use crate::app::State;
use crate::localization::LANGUAGE;
use crate::themes::AppTheme;
use eframe::egui;

#[derive(Default)]
pub struct FrontendWindow;

impl FrontendWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Background Image").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut state.launcher.show_background_image, "Show game background image");
                ui.add(
                    egui::Slider::new(&mut state.launcher.background_image_opacity, 0..=100)
                        .text("Opacity (%)"),
                );
            });
            columns[0].label(egui::RichText::new("Title Music").strong());
            if state.launcher.play_background_music {
                columns[0].label("With this option enabled, the launcher uses more RAM");
            }
            columns[0].group(|ui| {
                ui.checkbox(&mut state.launcher.play_background_music, "Play title music");
                ui.add(
                    egui::Slider::new(&mut state.launcher.background_music_volume, 0..=100)
                        .text("Volume (%)"),
                );
            });
            columns[1].label(egui::RichText::new("Game Compatibility").strong());
            columns[1].group(|ui| {
                ui.checkbox(&mut state.launcher.show_compatibility, "Display compatibility data");
                ui.checkbox(&mut state.launcher.check_compatibility_at_startup, "Update on startup");
            });
            columns[1].label(egui::RichText::new("GUI").strong());
            columns[1].group(|ui| {
                ui.checkbox(&mut state.launcher.check_for_updates, "Check for updates at startup");
                ui.checkbox(&mut state.launcher.show_change_log, "Show change log");
                ui.add_enabled(false, egui::Button::new("Check for updates"))
                    .on_disabled_hover_text("The updater is not available yet.");
                ui.label(egui::RichText::new("Theme").strong());
                ui.group(|ui| {
                    let current = &mut state.launcher.theme;
                    egui::ComboBox::from_id_salt("launcher_theme")
                        .width(ui.available_width())
                        .selected_text(
                            AppTheme::from_id(*current).map_or_else(|| "Unknown".to_owned(), AppTheme::label),
                        )
                        .show_ui(ui, |ui| {
                            for app_theme in AppTheme::ALL {
                                ui.selectable_value(current, *app_theme as u32, AppTheme::label(*app_theme));
                            }
                        });
                });
                ui.label(egui::RichText::new("Default Settings Tab").strong());
                ui.group(|ui| {
                    let value = &mut state.launcher.home_tab;
                    let options = &[
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
                    ];
                    egui::ComboBox::from_id_salt("default_settings_tab")
                        .selected_text(
                            options
                                .iter()
                                .find(|(_, stored)| *stored == value)
                                .map_or(value.as_str(), |(display, _)| display),
                        )
                        .show_ui(ui, |ui| {
                            for (display, stored) in options {
                                ui.selectable_value(value, (*stored).to_owned(), *display);
                            }
                        });
                });
            });
        });
    }
}
