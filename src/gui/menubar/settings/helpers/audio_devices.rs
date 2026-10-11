#[derive(Default)]
pub(in crate::gui::menubar::settings) struct AudioDevices {
    pub backend: u64,
    pub outputs: Vec<String>,
    pub inputs: Vec<String>,
    pub error: Option<String>,
}

impl AudioDevices {
    pub fn scan(backend: u64) -> Self {
        let result = if backend == 1 { scan_openal() } else { scan_sdl() };
        match result {
            Ok((outputs, inputs)) => Self {
                backend,
                outputs,
                inputs,
                error: None,
            },
            Err(error) => Self {
                backend,
                error: Some(error),
                ..Self::default()
            },
        }
    }
}

fn scan_sdl() -> Result<(Vec<String>, Vec<String>), String> {
    let sdl = sdl3::init().map_err(|error| error.to_string())?;
    let audio = sdl.audio().map_err(|error| error.to_string())?;

    let outputs = audio
        .audio_playback_device_ids()
        .map_err(|error| error.to_string())?
        .into_iter()
        .filter_map(|device| device.name().ok())
        .collect();
    let inputs = audio
        .audio_recording_device_ids()
        .map_err(|error| error.to_string())?
        .into_iter()
        .filter_map(|device| device.name().ok())
        .collect();

    Ok((outputs, inputs))
}

fn scan_openal() -> Result<(Vec<String>, Vec<String>), String> {
    let alto = alto::Alto::load_default().map_err(|error| error.to_string())?;
    let outputs = alto
        .enumerate_outputs()
        .into_iter()
        .map(|name| name.to_string_lossy().into_owned())
        .collect();
    let inputs = alto
        .enumerate_captures()
        .into_iter()
        .map(|name| name.to_string_lossy().into_owned())
        .collect();

    Ok((outputs, inputs))
}
