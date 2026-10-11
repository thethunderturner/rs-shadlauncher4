use crate::common::config::{Config, InstallDir};
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| section(ui, "Game Folders", |ui| game_folders(ui, &mut config.general.install_dirs)),
        |ui, config| {
            section(ui, "DLC Folder", |ui| path_buf(ui, &mut config.general.addon_install_dir));
            section(ui, "PS4 Sysmodules", |ui| path(ui, &mut config.general.sys_modules_dir));
            section(ui, "Home Folder", |ui| path(ui, &mut config.general.home_dir));
        },
    );
}

fn game_folders(ui: &mut egui::Ui, dirs: &mut Vec<InstallDir>) {
    let mut remove = None;
    for (index, dir) in dirs.iter_mut().enumerate() {
        ui.push_id(index, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut dir.enabled, "");
                let mut path = dir.path.to_string_lossy().into_owned();
                if ui.text_edit_singleline(&mut path).changed() {
                    dir.path = path.into();
                }
                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        dir.path = path;
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
            dirs.push(InstallDir { enabled: true, path });
        }
    }
}
