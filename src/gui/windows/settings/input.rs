use crate::config::emulator::Config;
use crate::devices::camera::CameraDevices;
use eframe::egui;

#[derive(Default)]
pub struct InputWindow {
    camera_devices: CameraDevices,
}

impl InputWindow {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            camera_devices: CameraDevices::scan(),
        })
    }

    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Controller").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.input.motion_controls_enabled, "Enable motion controls");
                ui.checkbox(&mut config.input.background_controller_input, "Enable background input");
            });
            columns[0].label(egui::RichText::new("Cursor").strong());
            columns[0].group(|ui| {
                ui.label(egui::RichText::new("Hide Cursor").strong());
                ui.group(|ui| {
                    let value = &mut config.input.cursor_state;
                    egui::ComboBox::from_id_salt("hide_cursor")
                        .selected_text(match value {
                            0 => "Never",
                            1 => "Idle",
                            2 => "Always",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, 0, "Never");
                            ui.selectable_value(value, 1, "Idle");
                            ui.selectable_value(value, 2, "Always");
                        });
                });
                ui.label(egui::RichText::new("Hide Cursor Idle Timeout").strong());
                ui.group(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut config.input.cursor_hide_timeout)
                            .range(0..=3600)
                            .suffix("s"),
                    );
                });
            });

            columns[1].label(egui::RichText::new("Devices").strong());
            columns[1].group(|ui| {
                ui.label(egui::RichText::new("Camera").strong());
                ui.group(|ui| {
                    let current = &mut config.input.camera_id;
                    egui::ComboBox::from_id_salt("camera_device")
                        .selected_text(if *current == -1 {
                            "None"
                        } else {
                            self.camera_devices
                                .devices
                                .get(*current as usize)
                                .map(String::as_str)
                                .unwrap_or("Unknown")
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(current, -1, "None");
                            for (id, camera) in self.camera_devices.devices.iter().enumerate() {
                                ui.selectable_value(current, id as i32, camera);
                            }
                        });
                });
                ui.label(egui::RichText::new("USB Device").strong());
                ui.group(|ui| {
                    let value = &mut config.input.usb_device_backend;
                    egui::ComboBox::from_id_salt("usb_backend")
                        .selected_text(match value {
                            0 => "Real USB Device",
                            1 => "Skylanders Portal",
                            2 => "Infinity Base",
                            3 => "Dimensions Toypad",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, 0, "Real USB Device");
                            ui.selectable_value(value, 1, "Skylanders Portal");
                            ui.selectable_value(value, 2, "Infinity Base");
                            ui.selectable_value(value, 3, "Dimensions Toypad");
                        });
                });
            });
        });
    }
}
