use eframe::egui;
use std::sync::mpsc::{self, Receiver, TryRecvError};

#[cfg(not(target_os = "macos"))]
use ash::{Entry, vk};
#[cfg(not(target_os = "macos"))]
use std::ffi::CStr;

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {}

#[derive(Default)]
pub struct GpuDevices {
    names: Vec<String>,
    receiver: Option<Receiver<Result<Vec<String>, String>>>,
    error: Option<String>,
    scanned: bool,
}

impl GpuDevices {
    pub fn start_scan(&mut self, ctx: &egui::Context) {
        if self.receiver.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        self.names.clear();
        self.error = None;
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result = enumerate();
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }

    pub fn ensure_scan(&mut self, ctx: &egui::Context) {
        if !self.scanned && self.receiver.is_none() {
            self.start_scan(ctx);
        }
    }

    pub fn poll(&mut self) {
        let Some(receiver) = &self.receiver else {
            return;
        };
        match receiver.try_recv() {
            Ok(Ok(names)) => {
                self.names = names;
                self.error = None;
                self.scanned = true;
                self.receiver = None;
            }
            Ok(Err(error)) => {
                self.names.clear();
                self.error = Some(error);
                self.scanned = true;
                self.receiver = None;
            }
            Err(TryRecvError::Disconnected) => {
                self.error = Some("GPU discovery stopped unexpectedly.".to_owned());
                self.scanned = true;
                self.receiver = None;
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

#[cfg(not(target_os = "macos"))]
fn enumerate() -> Result<Vec<String>, String> {
    // Vulkan is loaded at runtime so a missing driver does not stop the launcher from opening.
    let entry =
        unsafe { Entry::load() }.map_err(|error| format!("Vulkan is unavailable: {error}"))?;
    let app_info = vk::ApplicationInfo {
        api_version: vk::API_VERSION_1_3,
        ..Default::default()
    };
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };
    let instance = unsafe { entry.create_instance(&create_info, None) }
        .map_err(|error| format!("Could not start Vulkan: {error:?}"))?;
    let result = unsafe { instance.enumerate_physical_devices() }
        .map(|devices| {
            devices
                .into_iter()
                .map(|device| {
                    let properties = unsafe { instance.get_physical_device_properties(device) };
                    unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
                        .to_string_lossy()
                        .into_owned()
                })
                .collect()
        })
        .map_err(|error| format!("Could not list Vulkan GPUs: {error:?}"));
    unsafe { instance.destroy_instance(None) };
    result
}

#[cfg(target_os = "macos")]
fn enumerate() -> Result<Vec<String>, String> {
    use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

    Ok(MTLCreateSystemDefaultDevice()
        .map(|device| vec![device.name().to_string()])
        .unwrap_or_default())
}
