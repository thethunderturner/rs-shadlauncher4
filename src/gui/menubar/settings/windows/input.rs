use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::camera_devices::CameraDevices;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(
    ui: &mut egui::Ui,
    config: &mut Config,
    camera_devices: &mut CameraDevices,
) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Controller", |ui| {
                check(ui, &mut config.input.motion_controls_enabled, "Enable motion controls");
                check(ui, &mut config.input.background_controller_input, "Enable background input");
            });
            section(ui, "Cursor", |ui| {
                index(
                    ui,
                    &mut config.input.cursor_state,
                    "cursor_state",
                    "Hide cursor",
                    &["Never", "Idle", "Always"],
                );
                if config.input.cursor_state == 1 {
                    uint(ui, &mut config.input.cursor_hide_timeout, "Idle timeout (seconds)", 0..=3600);
                }
            });
        },
        |ui, config| {
            section(ui, "Devices", |ui| {
                index(
                    ui,
                    &mut config.input.usb_device_backend,
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

fn camera_picker(ui: &mut egui::Ui, config: &mut Config, devices: &mut CameraDevices) {
    devices.ensure_scan(ui.ctx());
    let selected = &mut config.input.camera_id;
    let selected_name = if *selected == -1 {
        "None".to_owned()
    } else {
        usize::try_from(*selected)
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
                    ui.selectable_value(selected, -1, "None");
                    for (index, name) in devices.names().iter().enumerate() {
                        ui.selectable_value(selected, index as i32, name);
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
}
