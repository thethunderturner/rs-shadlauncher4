use crate::config::emulator::{Config, InstallDir};
use eframe::egui;

#[derive(Default)]
pub struct PathsWindow;

impl PathsWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Game Folders").strong());
            columns[0].group(|ui| {
                let mut remove = None;
                let dirs = &mut config.general.install_dirs;
                for (index, dir) in dirs.iter_mut().enumerate() {
                    ui.push_id(index, |ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut dir.enabled, "");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Remove").clicked() {
                                    remove = Some(index);
                                }
                                if ui.button("Browse").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        dir.path = path;
                                    }
                                }
                                let mut path = dir.path.to_string_lossy().into_owned();
                                ui.add(
                                    egui::TextEdit::singleline(&mut path)
                                        .interactive(false)
                                        .desired_width(ui.available_width().max(0.0)),
                                );
                            });
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
            });

            columns[1].label(egui::RichText::new("DLC Path").strong());
            columns[1].group(|ui| {
                ui.horizontal(|ui| {
                    let mut path = config.general.addon_install_dir.to_string_lossy().into_owned();
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Browse").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                config.general.addon_install_dir = folder;
                            }
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut path)
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .background_color(ui.visuals().extreme_bg_color),
                        );
                    });
                });
            });
            columns[1]
                .label(egui::RichText::new("Home Folder Location (Saves, Inputs, Trophy Data)").strong());
            columns[1].group(|ui| {
                ui.horizontal(|ui| {
                    let mut path = config.general.home_dir.to_string_lossy().into_owned();
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Browse").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                config.general.home_dir = folder;
                            }
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut path)
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .background_color(ui.visuals().extreme_bg_color),
                        );
                    });
                });
            });
            columns[1].label(egui::RichText::new("PS4 Sysmodules Path").strong());
            columns[1].group(|ui| {
                ui.horizontal(|ui| {
                    let mut path = config.general.sys_modules_dir.to_string_lossy().into_owned();
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Browse").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                config.general.sys_modules_dir = folder;
                            }
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut path)
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .background_color(ui.visuals().extreme_bg_color),
                        );
                    });
                });
            });
            columns[1].label(egui::RichText::new("PS4 Fonts Path").strong());
            columns[1].group(|ui| {
                ui.horizontal(|ui| {
                    let mut path = config.general.font_dir.to_string_lossy().into_owned();
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Browse").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                config.general.font_dir = folder;
                            }
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut path)
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .background_color(ui.visuals().extreme_bg_color),
                        );
                    });
                });
            });
        });
    }
}
