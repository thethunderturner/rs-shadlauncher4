//! Safe owner for the vendored LibAtrac9 decoder.
use std::ffi::{c_int, c_void};
use std::ptr::NonNull;
use std::sync::Mutex;

// LibAtrac9 initializes shared lookup tables; serialize initialization and decoding.
static CODEC_LOCK: Mutex<()> = Mutex::new(());

#[repr(C)]
#[derive(Default)]
struct RawInfo {
    channels: c_int,
    channel_config_index: c_int,
    sampling_rate: c_int,
    superframe_size: c_int,
    frames_in_superframe: c_int,
    frame_samples: c_int,
    word_length: c_int,
    config: [u8; 4],
}

unsafe extern "C" {
    fn Atrac9GetHandle() -> *mut c_void;
    fn Atrac9ReleaseHandle(handle: *mut c_void);
    fn Atrac9InitDecoder(handle: *mut c_void, config: *mut u8) -> c_int;
    fn Atrac9GetCodecInfo(handle: *mut c_void, info: *mut RawInfo) -> c_int;
    fn Atrac9Decode(
        handle: *mut c_void,
        input: *const u8,
        input_size: c_int,
        output: *mut i16,
        bytes_used: *mut c_int,
        no_interleave: c_int,
    ) -> c_int;
}

#[derive(Clone, Copy)]
pub(in crate::gui::centralpanel::actions::row) struct CodecInfo {
    pub channels: usize,
    pub sampling_rate: u32,
    pub superframe_size: usize,
    pub frames_in_superframe: usize,
    pub frame_samples: usize,
}

pub(in crate::gui::centralpanel::actions::row) struct Atrac9Decoder {
    handle: NonNull<c_void>,
    info: CodecInfo,
}

impl Atrac9Decoder {
    pub fn new(config: &[u8; 4]) -> Result<Self, String> {
        // The C channel-configuration table has six entries.
        if config[0] != 0xFE || config[1] & 1 != 0 || (config[1] >> 1) & 7 >= 6 {
            return Err("Invalid ATRAC9 configuration".into());
        }
        let _lock = CODEC_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        // SAFETY: allocation returns an owned opaque handle, checked for null.
        let handle = NonNull::new(unsafe { Atrac9GetHandle() })
            .ok_or("Could not allocate ATRAC9 decoder")?;
        let mut config = *config;
        let mut raw = RawInfo::default();
        // SAFETY: live handle, four configuration bytes, and correctly laid-out output struct.
        let status = unsafe {
            let status = Atrac9InitDecoder(handle.as_ptr(), config.as_mut_ptr());
            if status == 0 {
                Atrac9GetCodecInfo(handle.as_ptr(), &mut raw)
            } else {
                status
            }
        };
        if status != 0
            || !(1..=8).contains(&raw.channels)
            || !(1..=256).contains(&raw.frame_samples)
            || !(1..=8).contains(&raw.frames_in_superframe)
            || !(1..=16384).contains(&raw.superframe_size)
            || raw.sampling_rate <= 0
        {
            // SAFETY: release the allocation on initialization failure exactly once.
            unsafe { Atrac9ReleaseHandle(handle.as_ptr()) };
            return Err(format!("Invalid ATRAC9 codec configuration ({status:#x})"));
        }
        Ok(Self {
            handle,
            info: CodecInfo {
                channels: raw.channels as usize,
                sampling_rate: raw.sampling_rate as u32,
                superframe_size: raw.superframe_size as usize,
                frames_in_superframe: raw.frames_in_superframe as usize,
                frame_samples: raw.frame_samples as usize,
            },
        })
    }

    pub fn codec_info(&self) -> CodecInfo {
        self.info
    }

    pub fn decode(&mut self, input: &[u8], output: &mut [i16]) -> Result<(), String> {
        let info = self.info;
        let frame_samples = info.channels * info.frame_samples;
        if input.len() != info.superframe_size
            || output.len() != frame_samples * info.frames_in_superframe
        {
            return Err("Incorrect ATRAC9 buffer size".into());
        }
        let _lock = CODEC_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let mut offset = 0;
        for frame in output.chunks_exact_mut(frame_samples) {
            if offset >= input.len() {
                return Err("Truncated ATRAC9 superframe".into());
            }
            let mut used = 0;
            // SAFETY: handle is initialized and exclusively borrowed. Input length is
            // bounded; output holds exactly one interleaved frame for this configuration.
            let status = unsafe {
                Atrac9Decode(
                    self.handle.as_ptr(),
                    input[offset..].as_ptr(),
                    (input.len() - offset) as c_int,
                    frame.as_mut_ptr(),
                    &mut used,
                    0,
                )
            };
            if status != 0 || used <= 0 || used as usize > input.len() - offset {
                return Err(format!("ATRAC9 decoding failed ({status:#x})"));
            }
            offset += used as usize;
        }
        Ok(())
    }
}

impl Drop for Atrac9Decoder {
    fn drop(&mut self) {
        // SAFETY: this owner releases its live handle exactly once.
        unsafe { Atrac9ReleaseHandle(self.handle.as_ptr()) };
    }
}
