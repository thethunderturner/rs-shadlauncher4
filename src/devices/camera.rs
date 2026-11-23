use sdl3_sys::camera::{SDL_GetCameraName, SDL_GetCameras};
use sdl3_sys::stdinc::SDL_free;
use std::ffi::CStr;

#[derive(Default, Debug)]
pub struct CameraDevices {
    pub devices: Vec<String>,
    pub error: Option<String>,
}

impl CameraDevices {
    pub fn scan() -> Self {
        match scan_cameras() {
            Ok(devices) => Self { devices, error: None },
            Err(error) => Self {
                error: Some(error),
                ..Self::default()
            },
        }
    }
}

fn scan_cameras() -> Result<Vec<String>, String> {
    let sdl = sdl3::init().map_err(|error| error.to_string())?;
    let _camera = sdl.camera().map_err(|error| error.to_string())?;

    unsafe {
        let mut count = 0;
        let cameras = SDL_GetCameras(&mut count);
        if cameras.is_null() {
            return Err(sdl3::get_error().to_string());
        }

        let devices = std::slice::from_raw_parts(cameras, count as usize)
            .iter()
            .filter_map(|&id| {
                let name = SDL_GetCameraName(id);
                if name.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(name).to_string_lossy().into_owned())
                }
            })
            .collect();

        SDL_free(cameras.cast());

        Ok(devices)
    }
}
