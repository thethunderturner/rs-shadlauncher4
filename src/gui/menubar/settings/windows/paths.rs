use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::{Value, json};

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| section(ui, "Game Folders", |ui| game_folders(ui, config)),
        |ui, config| {
            section(ui, "DLC Folder", |ui| {
                path(ui, config, "General", "addon_install_dir")
            });
            section(ui, "PS4 Sysmodules", |ui| {
                path(
                    ui,
                    config,
                    "General",
                    "sys_modules_dir"
                )
            });
            section(ui, "Home Folder", |ui| {
                path(ui, config, "General", "home_dir")
            });
        },
    );
}

fn game_folders(ui: &mut egui::Ui, config: &mut Value) {
    let dirs = &mut config["General"]["install_dirs"];
    if !dirs.is_array() {
        *dirs = json!([]);
    }
    let dirs = dirs.as_array_mut().expect("install_dirs is an array");
    let mut remove = None;
    for (index, dir) in dirs.iter_mut().enumerate() {
        ui.push_id(index, |ui| {
            ui.horizontal(|ui| {
                let mut enabled = dir["enabled"].as_bool().unwrap_or(true);
                if ui.checkbox(&mut enabled, "").changed() {
                    dir["enabled"] = json!(enabled);
                }
                let mut path = dir["path"].as_str().unwrap_or("").to_owned();
                if ui.text_edit_singleline(&mut path).changed() {
                    dir["path"] = json!(path);
                }
                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        dir["path"] = json!(path.to_string_lossy());
                    }
                }
                if ui.button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        });
    }
    if let Some(index) = remove {
        dirs.remove(index);
    }
    if ui.button("Add folder").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            dirs.push(json!({"enabled": true, "path": path.to_string_lossy()}));
        }
    }
}
