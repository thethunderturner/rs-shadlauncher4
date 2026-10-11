use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Trophy Notifications", |ui| {
                check(ui, &mut config.general.trophy_popup_disabled, "Disable trophy pop-ups");
                decimal(
                    ui,
                    &mut config.general.trophy_notification_duration,
                    "Duration (seconds)",
                    0.0..=60.0,
                );
                choice(
                    ui,
                    &mut config.general.trophy_notification_side,
                    "trophy_notification_side",
                    "Position",
                    &[
                        ("Top", "top"),
                        ("Bottom", "bottom"),
                        ("Left", "left"),
                        ("Right", "right"),
                    ],
                );
            });
        },
        |ui, _| {
            section(ui, "Custom Trophies", |ui| {
                ui.label("Custom images and sounds can be placed in user/custom_trophy.");
                if ui.button("Open custom trophy folder").clicked() {
                    if let Ok(path) = crate::common::user_directory() {
                        let _ = open_folder(path.join("custom_trophy"));
                    }
                }
            });
            section(ui, "Portable User Folder", |ui| {
                ui.add_enabled(false, egui::Button::new("Create portable launcher folder"))
                    .on_disabled_hover_text("Portable launcher folders are not supported yet.");
            });
        },
    );
}

pub(in crate::gui::menubar::settings) fn open_folder(path: std::path::PathBuf) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    let mut command = std::process::Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut command = std::process::Command::new("open");
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let mut command = std::process::Command::new("xdg-open");
    command.arg(path).spawn().map(|_| ())
}
