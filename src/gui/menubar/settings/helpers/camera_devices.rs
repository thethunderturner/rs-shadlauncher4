use eframe::egui;
use sdl3::sys::{camera, stdinc};
use std::ffi::CStr;
use std::sync::mpsc::{self, Receiver, TryRecvError};

// Could go for https://docs.rs/nokhwa/latest/nokhwa/fn.query.html. I prefer SDL3, because thats how the QT emulator enumerates devices, and i dont want to mess something up.
#[derive(Default)]
pub struct CameraDevices {
    names: Vec<String>,
    receiver: Option<Receiver<Result<Vec<String>, String>>>,
    worker: Option<std::thread::JoinHandle<()>>,
    subsystem: Option<sdl3::CameraSubsystem>,
    error: Option<String>,
    scanned: bool,
}

impl CameraDevices {
    pub fn ensure_scan(&mut self, ctx: &egui::Context) {
        if !self.scanned && self.receiver.is_none() {
            self.start_scan(ctx);
        }
    }

    pub fn start_scan(&mut self, ctx: &egui::Context) {
        if self.receiver.is_some() {
            return;
        }
        self.names.clear();
        self.error = None;
        self.scanned = true;

        let subsystem = match sdl3::init().and_then(|sdl| sdl.camera()) {
            Ok(subsystem) => subsystem,
            Err(error) => {
                self.error = Some(format!("Could not initialize cameras: {error}"));
                return;
            }
        };
        self.subsystem = Some(subsystem);

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        let ctx = ctx.clone();
        self.worker = Some(std::thread::spawn(move || {
            let _ = tx.send(enumerate());
            ctx.request_repaint();
        }));
    }

    pub fn poll(&mut self) {
        let Some(receiver) = &self.receiver else {
            return;
        };
        match receiver.try_recv() {
            Ok(Ok(names)) => {
                self.names = names;
                self.error = None;
                self.receiver = None;
                self.worker = None;
                self.subsystem = None;
            }
            Ok(Err(error)) => {
                self.error = Some(error);
                self.receiver = None;
                self.worker = None;
                self.subsystem = None;
            }
            Err(TryRecvError::Disconnected) => {
                self.error = Some("Camera discovery stopped unexpectedly.".to_owned());
                self.receiver = None;
                self.worker = None;
                self.subsystem = None;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn is_scanning(&self) -> bool {
        self.receiver.is_some()
    }
}

impl Drop for CameraDevices {
    fn drop(&mut self) {
        // Keep SDL's camera subsystem alive until the worker stops using it.
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn enumerate() -> Result<Vec<String>, String> {
    let mut count = 0;
    sdl3::clear_error();
    let devices = unsafe { camera::SDL_GetCameras(&mut count) };
    if devices.is_null() {
        let error = sdl3::get_error();
        return if error.is_empty() {
            Ok(Vec::new())
        } else {
            Err(format!("Could not list cameras: {error}"))
        };
    }

    // SDL owns the returned array. Copy names while it is valid, then free it once.
    let names = unsafe { std::slice::from_raw_parts(devices, count as usize) }
        .iter()
        .enumerate()
        .map(|(index, &device)| {
            let name = unsafe { camera::SDL_GetCameraName(device) };
            if name.is_null() {
                format!("Unknown camera ({index})")
            } else {
                unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned()
            }
        })
        .collect();
    unsafe { stdinc::SDL_free(devices.cast()) };
    Ok(names)
}
