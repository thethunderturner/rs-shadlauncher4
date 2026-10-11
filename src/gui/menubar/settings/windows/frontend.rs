use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Background Image", |ui| {
                check(
                    ui,
                    &mut config.frontend.show_background_image,
                    "Show game background image",
                );
                uint(
                    ui,
                    &mut config.frontend.background_image_opacity,
                    "Opacity (%)",
                    0..=100,
                );
            });
            section(ui, "Title Music", |ui| {
                check(
                    ui,
                    &mut config.frontend.play_background_music,
                    "Play title music",
                );
                uint(
                    ui,
                    &mut config.frontend.background_music_volume,
                    "Volume (%)",
                    0..=100,
                );
            });
        },
        |ui, config| {
            section(ui, "Compatibility", |ui| {
                check(
                    ui,
                    &mut config.frontend.show_compatibility,
                    "Display compatibility data",
                );
                check(
                    ui,
                    &mut config.frontend.check_compatibility_at_startup,
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
                    &mut config.frontend.check_for_updates,
                    "Check for updates",
                );
                check(ui, &mut config.frontend.show_change_log, "Show change log");
                ui.add_enabled(false, egui::Button::new("Check now"))
                    .on_disabled_hover_text("The updater is not available yet.");
            });
            section(ui, "Settings Window", |ui| {
                choice(
                    ui,
                    &mut config.frontend.home_tab,
                    "home_tab",
                    "Default tab",
                    &[
                        ("General", "General"),
                        ("Frontend", "Frontend"),
                        ("Graphics", "Graphics"),
                        ("User", "User"),
                        ("Input", "Input"),
                        ("Network", "Network"),
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
