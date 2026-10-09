use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Logging", |ui| {
                check(ui, config, "Log", "enable", "Enable logging");
                text(ui, config, "Log", "filter", "Log filter");
                ui.menu_button("Log filter presets", |ui| {
                    for (label, filter) in [
                        ("Default", "*:Info"),
                        ("Warnings and errors", "*:Warning"),
                        ("Debug", "*:Debug"),
                        ("Trace", "*:Trace"),
                    ] {
                        if ui.button(label).clicked() {
                            config["Log"]["filter"] = Value::String(filter.to_owned());
                            ui.close();
                        }
                    }
                });
                choice(
                    ui,
                    config,
                    "Log",
                    "flush_level",
                    "Flush level",
                    &[
                        ("Trace", "trace"),
                        ("Debug", "debug"),
                        ("Info", "info"),
                        ("Warning", "warning"),
                        ("Error", "error"),
                        ("Critical", "critical"),
                    ],
                );
            });
            section(ui, "Duplicate Lines", |ui| {
                check(ui, config, "Log", "skip_duplicate", "Skip duplicate lines");
                if config["Log"]["skip_duplicate"].as_bool() == Some(true) {
                    uint(
                        ui,
                        config,
                        "Log",
                        "max_skip_duration",
                        "Maximum skip duration (ms)",
                        0..=u32::MAX as u64,
                    );
                }
            });
        },
        |ui, config| {
            section(ui, "Log Files", |ui| {
                check(ui, config, "Log", "append", "Append to existing logs");
                check(ui, config, "Log", "separate", "Separate log files per game");
                check(ui, config, "Log", "sync", "Synchronous logging");
                uint(
                    ui,
                    config,
                    "Log",
                    "size_limit",
                    "Size limit (bytes)",
                    0..=u64::MAX,
                );
                if ui.button("Open log folder").clicked() {
                    if let Ok(path) = crate::common::user_directory() {
                        let _ = super::user::open_folder(path.join("log"));
                    }
                }
            });
        },
    );
}
