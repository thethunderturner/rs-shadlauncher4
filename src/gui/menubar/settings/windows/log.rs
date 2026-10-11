use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Logging", |ui| {
                check(ui, &mut config.log.enable, "Enable logging");
                text(ui, &mut config.log.filter, "Log filter");
                ui.menu_button("Log filter presets", |ui| {
                    for (label, filter) in [
                        ("Default", "*:Info"),
                        ("Warnings and errors", "*:Warning"),
                        ("Debug", "*:Debug"),
                        ("Trace", "*:Trace"),
                    ] {
                        if ui.button(label).clicked() {
                            config.log.filter = filter.to_owned();
                            ui.close();
                        }
                    }
                });
                choice(
                    ui,
                    &mut config.log.flush_level,
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
                check(ui, &mut config.log.skip_duplicate, "Skip duplicate lines");
                if config.log.skip_duplicate {
                    uint(ui, &mut config.log.max_skip_duration, "Maximum skip duration (ms)", 0..=u32::MAX);
                }
            });
        },
        |ui, config| {
            section(ui, "Log Files", |ui| {
                check(ui, &mut config.log.append, "Append to existing logs");
                check(ui, &mut config.log.separate, "Separate log files per game");
                check(ui, &mut config.log.sync, "Synchronous logging");
                uint(ui, &mut config.log.size_limit, "Size limit (bytes)", 0..=u64::MAX);
                if ui.button("Open log folder").clicked() {
                    if let Ok(path) = crate::common::user_directory() {
                        let _ = super::user::open_folder(path.join("log"));
                    }
                }
            });
        },
    );
}
