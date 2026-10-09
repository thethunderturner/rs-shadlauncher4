use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Background Image", |ui| {
                check(
                    ui,
                    config,
                    "Frontend",
                    "show_background_image",
                    "Show game background image",
                );
                uint(
                    ui,
                    config,
                    "Frontend",
                    "background_image_opacity",
                    "Opacity (%)",
                    0..=100,
                );
            });
            section(ui, "Title Music", |ui| {
                check(
                    ui,
                    config,
                    "Frontend",
                    "play_background_music",
                    "Play title music",
                );
                uint(
                    ui,
                    config,
                    "Frontend",
                    "background_music_volume",
                    "Volume (%)",
                    0..=100,
                );
            });
        },
        |ui, config| {
            section(ui, "Compatibility", |ui| {
                check(
                    ui,
                    config,
                    "Frontend",
                    "show_compatibility",
                    "Display compatibility data",
                );
                check(
                    ui,
                    config,
                    "Frontend",
                    "check_compatibility_at_startup",
                    "Update on startup",
                );
                ui.add_enabled(false, egui::Button::new("Update compatibility database"))
                    .on_disabled_hover_text(
                        "Compatibility database updates are not available yet.",
                    );
            });
            section(ui, "Updates", |ui| {
                check(
                    ui,
                    config,
                    "Frontend",
                    "check_for_updates",
                    "Check for updates",
                );
                check(ui, config, "Frontend", "show_change_log", "Show change log");
                ui.add_enabled(false, egui::Button::new("Check now"))
                    .on_disabled_hover_text("The updater is not available yet.");
            });
            section(ui, "Settings Window", |ui| {
                choice(
                    ui,
                    config,
                    "Frontend",
                    "home_tab",
                    "Default tab",
                    &[
                        ("General", "General"),
                        ("Frontend", "Frontend"),
                        ("Graphics", "Graphics"),
                        ("User", "User"),
                        ("Input", "Input"),
                        ("Paths", "Paths"),
                        ("Log", "Log"),
                        ("Debug", "Debug"),
                        ("Experimental", "Experimental"),
                    ],
                );
            });
        },
    );
}
