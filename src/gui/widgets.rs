use crate::app::State;
use crate::library::cache::{LoadedTitle, LoadedTitles};
use crate::library::title::Title;
use eframe::egui;
use eframe::egui::Ui;

pub struct RowWidget;
pub struct PanelWidget;

impl RowWidget {
    pub fn label(ui: &mut Ui, str: &str) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(8.0);
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

    pub fn dialog(ui: &mut Ui, state: &State, loaded_title: &LoadedTitle) {
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
            if loaded_title.has_custom_config {
                if ui.button("Configure Game-Specific settings").clicked() {
                    ui.close();
                } else if ui.button("Delete Game-Specific settings").clicked() {
                    ui.close();
                }
            } else {
                if ui
                    .button("Create Game-Specific config from global settings")
                    .clicked()
                {
                    ui.close();
                }
                if ui
                    .button("Create Game-Specific config from default settings")
                    .clicked()
                {
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
