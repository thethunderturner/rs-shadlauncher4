use crate::app::State;
use crate::config::emulator::Config;
use crate::gui::windows::settings::SettingsWindow;
use crate::library::cache::LoadedTitle;
use crate::library::title::Title;
use eframe::egui;
use eframe::egui::{Direction, Ui};

pub struct RowWidget;
pub struct PanelWidget;

impl RowWidget {
    pub fn icon(ui: &mut Ui, loaded_title: &LoadedTitle) {
        if let Some(icon) = &loaded_title.icon {
            ui.with_layout(
                egui::Layout::centered_and_justified(Direction::LeftToRight)
                    .with_main_align(egui::Align::Center),
                |ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            format!("bytes://{}/icon.png", loaded_title.title.serial),
                            icon.clone(),
                        )
                        .fit_to_exact_size(egui::vec2(55.0, 55.0))
                        .corner_radius(6),
                    );
                },
            );
        }
    }

    pub fn compatibility(ui: &mut Ui, status: Option<&str>) {
        let status = status.filter(|status| !status.is_empty()).unwrap_or("Unknown");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
            ui.painter().circle_filled(
                rect.center(),
                5.0,
                crate::library::compatibility::status_color(status),
            );
            ui.add(egui::Label::new(status).truncate().selectable(false));
        });
    }

    pub fn label(ui: &mut Ui, str: &str, is_custom: bool) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            if is_custom {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/settings.svg"))
                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                        .alt_text("Game-specific settings"),
                )
                .on_hover_text("Game-specific settings");
            }
            ui.add(
                egui::Label::new(str)
                    .truncate()
                    .halign(egui::Align::LEFT)
                    .selectable(false),
            );
        });
    }

    pub fn region(ui: &mut Ui, content_id: &str) {
        let (region, flag) = match content_id.get(..2) {
            Some("UP" | "UB" | "UT" | "US") => {
                ("USA", egui::include_image!("../assets/table/regions/us.svg"))
            }
            Some("EP" | "EB") => ("Europe", egui::include_image!("../assets/table/regions/eu.svg")),
            Some("JP" | "JB") => ("Japan", egui::include_image!("../assets/table/regions/jp.svg")),
            Some("HP" | "HB") => ("Asia", egui::include_image!("../assets/table/regions/cn.svg")),
            Some("IP") => ("World", egui::include_image!("../assets/table/regions/wr.svg")),
            _ => ("Unknown", egui::include_image!("../assets/table/regions/xx.svg")),
        };

        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.add(
                egui::Image::new(flag)
                    .fit_to_exact_size(egui::vec2(32.0, 32.0))
                    .alt_text(region),
            )
            .on_hover_text(region);
        });
    }

    pub fn features(ui: &mut Ui, title: &Title) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            if title.supports_neo {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/table/features/pro.svg"))
                        .fit_to_exact_size(egui::vec2(32.0, 32.0))
                        .tint(ui.visuals().text_color()),
                )
                .on_hover_text("Features for PS4 Pro");
            }

            if title.supports_vr {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/table/features/vr.svg"))
                        .fit_to_exact_size(egui::vec2(32.0, 32.0))
                        .tint(ui.visuals().text_color()),
                )
                .on_hover_text("VR Support/Required");
            }
        });
    }

    pub fn dialog(
        ui: &mut Ui,
        state: &State,
        loaded_title: &LoadedTitle,
        custom_settings_window: &mut SettingsWindow,
    ) {
        ui.menu_button("Launch", |ui| {
            if loaded_title.has_custom_config {
                let _ = ui.button("Launch with game specific configs (default)");
            }
            let _ = ui.button("Launch with global configs only");
            let _ = ui.button("Launch with default settings");
        });
        ui.menu_button("Open Folder", |ui| {
            let _ = ui.button("Game Folder");
            let _ = ui.button("Update folder");
            let _ = ui.button("Log Folder");
            ui.menu_button("Save Data folder", |ui| {
                let _ = ui.button("Unimplemented user 1");
                let _ = ui.button("Unimplemented user 2");
                let _ = ui.button("Unimplemented user 3");
                let _ = ui.button("Unimplemented user 4");
            });
        });
        ui.menu_button("Game Specific Settings", |ui| {
            let serial = &loaded_title.title.serial;

            if loaded_title.has_custom_config {
                if ui.button("Configure Game-Specific settings").clicked() {
                    match Config::load_custom(serial) {
                        Ok(config) => {
                            custom_settings_window.begin_custom_edit(serial, state, config);
                            custom_settings_window.open = true;
                            ui.close();
                        }
                        Err(error) => {
                            eprintln!("Failed to load settings for {serial}: {error}");
                        }
                    }
                }
                // if ui.button("Delete Game-Specific settings").clicked() {
                //     match Config::delete_custom(&loaded_title.title.serial) {
                //         Ok(()) => {
                //             // custom_config_serials.remove(&title.serial);
                //             ui.ctx().request_repaint();
                //             Ok(None)
                //         }
                //         Err(error) => {
                //             eprintln!("Could not delete config: {error}");
                //         }
                //     }
                // }
            } else {
                if ui
                    .button("Create Game-Specific config from Global settings")
                    .clicked()
                {
                    custom_settings_window.begin_custom_edit(serial, state, state.config.clone());
                    custom_settings_window.open = true;
                    ui.close();
                }

                if ui
                    .button("Create Game-Specific config from Default settings")
                    .clicked()
                {
                    custom_settings_window.begin_custom_edit(serial, state, Config::default());
                    custom_settings_window.open = true;
                    ui.close();
                }
            }
        });
        let _ = ui.button("Add to favorites");
        ui.menu_button("Add to folder", |ui| {
            let _ = ui.button("TODO");
        });
        let _ = ui.button("Cheats/Patches");
        let _ = ui.button("Trophy Viewer");
        ui.add_enabled(false, egui::Button::new("SFO Viewer"))
            .on_disabled_hover_text("SFO Viewer is not implemented yet.");
        ui.menu_button("Copy Info", |ui| {
            for (label, value) in [
                ("Name", &loaded_title.title.name),
                ("Serial", &loaded_title.title.serial),
                ("Publisher ID", &loaded_title.title.publisher_id),
            ] {
                if ui.button(label).clicked() {
                    ui.ctx().copy_text(value.clone());
                    ui.close();
                }
            }
        });
        ui.menu_button("Delete", |ui| {
            let _ = ui.button("TODO");
        });
        ui.menu_button("Compatibility", |ui| {
            let _ = ui.button("TODO");
        });
        ui.menu_button("ZAR compression", |ui| {
            let _ = ui.button("TODO");
        });
    }
}
