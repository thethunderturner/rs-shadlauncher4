use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
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
                int(
                    ui,
                    config,
                    "Input",
                    "camera_id",
                    "Camera ID (-1: none)",
                    -1..=64,
                );
            });
        },
    );
}
