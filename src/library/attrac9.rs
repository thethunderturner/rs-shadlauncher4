use super::scan::Title;
use alto::Source;
use std::{
    ffi::c_void,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
};

static DECODER_LOCK: Mutex<()> = Mutex::new(());

#[repr(C)]
#[derive(Default)]
struct CodecInfo {
    channels: i32,
    channel_config_index: i32,
    sampling_rate: i32,
    superframe_size: i32,
    frames_in_superframe: i32,
    frame_samples: i32,
    wlength: i32,
    config_data: [u8; 4],
}

unsafe extern "C" {
    fn Atrac9GetHandle() -> *mut c_void;
    fn Atrac9ReleaseHandle(handle: *mut c_void);
    fn Atrac9InitDecoder(handle: *mut c_void, config: *mut u8) -> i32;
    fn Atrac9GetCodecInfo(handle: *mut c_void, info: *mut CodecInfo) -> i32;
    fn Atrac9Decode(
        handle: *mut c_void,
        data: *const u8,
        size: i32,
        pcm: *mut i16,
        used: *mut i32,
        no_interleave: i32,
    ) -> i32;
}

struct Decoder(*mut c_void);
impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { Atrac9ReleaseHandle(self.0) };
    }
}

struct Audio {
    samples: Vec<i16>,
    channels: i32,
    rate: i32,
}

fn chunk<'a>(bytes: &'a [u8], id: &[u8; 4]) -> Result<&'a [u8], String> {
    if bytes.get(..4) != Some(b"RIFF") || bytes.get(8..12) != Some(b"WAVE") {
        return Err("Expected an ATRAC9 RIFF/WAVE file".into());
    }
    let mut offset = 12;
    while let Some(header) = bytes.get(offset..offset + 8) {
        let size = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let start = offset + 8;
        let end = start.checked_add(size).ok_or("Invalid RIFF chunk size")?;
        let data = bytes.get(start..end).ok_or("Truncated RIFF chunk")?;
        if &header[..4] == id {
            return Ok(data);
        }
        offset = end.checked_add(size % 2).ok_or("Invalid RIFF padding")?;
    }
    Err(format!("Missing {} chunk", String::from_utf8_lossy(id)))
}

fn decode(bytes: &[u8], cancelled: impl Fn() -> bool) -> Result<Option<Audio>, String> {
    let fmt = chunk(bytes, b"fmt ")?;
    // WAVEFORMATEXTENSIBLE ATRAC9 GUID, followed by version and codec config
    let guid = [
        0xd2, 0x42, 0xe1, 0x47, 0xba, 0x36, 0x8d, 0x4d, 0x88, 0xfc, 0x61, 0x65, 0x4f, 0x8c, 0x83, 0x6c,
    ];
    if fmt.get(..2) != Some(&[0xfe, 0xff]) || fmt.get(24..40) != Some(&guid) {
        return Err("Sound is not ATRAC9".into());
    }
    let mut config: [u8; 4] = fmt
        .get(44..48)
        .ok_or("Missing ATRAC9 config")?
        .try_into()
        .unwrap();
    // Only mono/stereo title sounds are supported; reject reserved channel configs
    if config[0] != 0xfe || (config[1] >> 1) & 7 > 2 {
        return Err("Unsupported ATRAC9 channel configuration".into());
    }
    let data = chunk(bytes, b"data")?;
    let _lock = DECODER_LOCK.lock().map_err(|_| "ATRAC9 decoder lock poisoned")?;
    // SAFETY: all pointers below refer to live, correctly sized input/output buffers
    let decoder = Decoder(unsafe { Atrac9GetHandle() });
    if decoder.0.is_null() {
        return Err("Could not allocate ATRAC9 decoder".into());
    }
    let mut info = CodecInfo::default();
    if unsafe { Atrac9InitDecoder(decoder.0, config.as_mut_ptr()) } != 0
        || unsafe { Atrac9GetCodecInfo(decoder.0, &mut info) } != 0
    {
        return Err("Could not initialize ATRAC9 decoder".into());
    }
    if !(1..=2).contains(&info.channels)
        || !(1..=256).contains(&info.frame_samples)
        || info.superframe_size <= 0
        || !(1..=8).contains(&info.frames_in_superframe)
        || info.sampling_rate <= 0
    {
        return Err("Unsupported ATRAC9 format".into());
    }
    let mut samples = Vec::new();
    let mut frame = vec![0i16; (info.channels * info.frame_samples) as usize];
    let mut blocks = data.chunks_exact(info.superframe_size as usize);
    for block in &mut blocks {
        if cancelled() {
            return Ok(None);
        }
        let mut remaining = block;
        for _ in 0..info.frames_in_superframe {
            let mut used = 0;
            let status = unsafe {
                Atrac9Decode(
                    decoder.0,
                    remaining.as_ptr(),
                    remaining.len() as i32,
                    frame.as_mut_ptr(),
                    &mut used,
                    0,
                )
            };
            if status != 0 || used <= 0 || used as usize > remaining.len() {
                return Err(format!("Invalid ATRAC9 frame ({status:#x})"));
            }
            samples.extend_from_slice(&frame);
            remaining = &remaining[used as usize..];
        }
    }
    if !blocks.remainder().is_empty() {
        return Err("Truncated ATRAC9 superframe".into());
    }
    // fact stores the audible sample count and encoder delay (per channel)
    if let Ok(fact) = chunk(bytes, b"fact") {
        if fact.len() >= 12 {
            let count = u32::from_le_bytes(fact[..4].try_into().unwrap()) as usize * info.channels as usize;
            let skip = u32::from_le_bytes(fact[8..12].try_into().unwrap()) as usize * info.channels as usize;
            if skip > samples.len() || count > samples.len() - skip {
                return Err("Invalid ATRAC9 sample count".into());
            }
            samples.drain(..skip);
            samples.truncate(count);
        }
    }
    if samples.is_empty() {
        return Err("Empty ATRAC9 sound".into());
    }
    Ok(Some(Audio {
        samples,
        channels: info.channels,
        rate: info.sampling_rate,
    }))
}

enum Command {
    Play {
        paths: Vec<PathBuf>,
        generation: u64,
        volume: u32,
    },
    Stop,
    Volume(u32),
}

#[derive(Default)]
pub struct Player {
    pub sender: Option<mpsc::Sender<Command>>,
    pub generation: Arc<AtomicU64>,
    pub volume: u32,
    pub active: bool,
}

impl Player {
    pub fn play(&mut self, title: &Title, volume: u32) {
        if self.sender.is_none() {
            let (sender, receiver) = mpsc::channel();
            let generation = self.generation.clone();
            std::thread::spawn(move || run(receiver, generation));
            self.sender = Some(sender);
        }
        let mut paths = Vec::new();
        if let Some(patch) = &title.patch_path {
            paths.push(patch.join("sce_sys/snd0.at9"));
            paths.push(patch.join("snd0.at9"));
        }
        if let Some(app) = &title.app_path {
            paths.push(app.join("sce_sys/snd0.at9"));
        }
        self.volume = volume;
        self.active = true;
        let generation = self.generation.fetch_add(1, Ordering::Relaxed) + 1;
        let _ = self.sender.as_ref().unwrap().send(Command::Play {
            paths,
            generation,
            volume,
        });
    }

    pub fn stop(&mut self) {
        if !self.active {
            return;
        }
        self.active = false;
        self.generation.fetch_add(1, Ordering::Relaxed);
        if let Some(sender) = &self.sender {
            let _ = sender.send(Command::Stop);
        }
    }

    pub fn set_volume(&mut self, volume: u32) {
        if self.volume != volume {
            self.volume = volume;
            if let Some(sender) = &self.sender {
                let _ = sender.send(Command::Volume(volume));
            }
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.stop();
    }
}

fn run(receiver: mpsc::Receiver<Command>, generation: Arc<AtomicU64>) {
    let mut context = None;
    let mut source: Option<alto::StaticSource> = None;
    for command in receiver {
        match command {
            Command::Stop => {
                source = None;
            }
            Command::Volume(volume) => {
                if let Some(source) = &mut source {
                    let _ = source.set_gain(volume.min(100) as f32 / 100.0);
                }
            }
            Command::Play {
                paths,
                generation: requested,
                volume,
            } => {
                source = None;
                let cancelled = || generation.load(Ordering::Relaxed) != requested;
                if cancelled() {
                    continue;
                }
                let result = (|| -> Result<Option<alto::StaticSource>, String> {
                    let Some(path) = paths.into_iter().find(|path| path.is_file()) else {
                        return Ok(None);
                    };
                    let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
                    let Some(audio) = decode(&bytes, cancelled)? else {
                        return Ok(None);
                    };
                    if cancelled() {
                        return Ok(None);
                    }
                    if context.is_none() {
                        let alto = alto::Alto::load_default().map_err(|error| error.to_string())?;
                        let device = alto.open(None).map_err(|error| error.to_string())?;
                        context = Some(device.new_context(None).map_err(|error| error.to_string())?);
                    }
                    let context = context.as_ref().unwrap();
                    let buffer = match audio.channels {
                        1 => context.new_buffer::<alto::Mono<i16>, _>(&audio.samples[..], audio.rate),
                        _ => context.new_buffer::<alto::Stereo<i16>, _>(&audio.samples[..], audio.rate),
                    }
                    .map_err(|error| error.to_string())?;
                    let mut source = context.new_static_source().map_err(|error| error.to_string())?;
                    source
                        .set_buffer(Arc::new(buffer))
                        .map_err(|error| error.to_string())?;
                    source
                        .set_gain(volume.min(100) as f32 / 100.0)
                        .map_err(|error| error.to_string())?;
                    if cancelled() {
                        return Ok(None);
                    }
                    source.play();
                    Ok(Some(source))
                })();
                match result {
                    Ok(playing) => source = playing,
                    Err(error) => eprintln!("Could not play title sound: {error}"),
                }
            }
        }
    }
}
