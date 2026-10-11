use crate::gui::menubar::settings::helpers::camera_devices::CameraDevices;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(
    ui: &mut egui::Ui,
    config: &mut Value,
    camera_devices: &mut CameraDevices,
) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Controller", |ui| {
                check(
                    ui,
                    config,
                    "Input",
                    "motion_controls_enabled",
                    "Enable motion controls",
                );
                check(
                    ui,
                    config,
                    "Input",
                    "background_controller_input",
                    "Enable background input",
                );
            });
            section(ui, "Cursor", |ui| {
                index(
                    ui,
                    config,
                    "Input",
                    "cursor_state",
                    "Hide cursor",
                    &["Never", "Idle", "Always"],
                );
                if config["Input"]["cursor_state"].as_u64() == Some(1) {
                    uint(
                        ui,
                        config,
                        "Input",
                        "cursor_hide_timeout",
                        "Idle timeout (seconds)",
                        0..=3600,
                    );
                }
            });
        },
        |ui, config| {
            section(ui, "Devices", |ui| {
                index(
                    ui,
                    config,
                    "Input",
                    "usb_device_backend",
                    "USB device",
                    &[
                        "Real USB Device",
                        "Skylander Portal",
                        "Infinity Base",
                        "Dimensions Toypad",
                    ],
                );
                camera_picker(ui, config, camera_devices);
            });
        },
    );
}

fn camera_picker(ui: &mut egui::Ui, config: &mut Value, devices: &mut CameraDevices) {
    devices.ensure_scan(ui.ctx());
    let mut selected = config["Input"]["camera_id"].as_i64().unwrap_or(-1);
    let selected_name = if selected == -1 {
        "None".to_owned()
    } else {
        usize::try_from(selected)
            .ok()
            .and_then(|index| devices.names().get(index))
            .cloned()
            .unwrap_or_else(|| format!("Unavailable camera (ID {selected})"))
    };
    ui.vertical(|ui| {
        ui.label("Camera");
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("camera_device")
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, -1, "None");
                    for (index, name) in devices.names().iter().enumerate() {
                        ui.selectable_value(&mut selected, index as i64, name);
                    }
                });
            if devices.is_scanning() {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Finding cameras");
                });
            } else if let Some(error) = devices.error() {
                ui.colored_label(ui.visuals().error_fg_color, error);
            } else if devices.names().is_empty() {
                ui.label("No cameras found.");
            }
            if ui
                .add_enabled(!devices.is_scanning(), egui::Button::new("Refresh"))
                .clicked()
            {
                devices.start_scan(ui.ctx());
            }
        });
    });
    config["Input"]["camera_id"] = Value::from(selected);
}
