pub mod helpers;
mod windows;

use crate::gui::menubar::settings::windows::general;
use eframe::egui;
use helpers::audio_devices::AudioDevices;
use helpers::camera_devices::CameraDevices;
use helpers::gpu_devices::GpuDevices;
use serde_json::Value;
use std::path::PathBuf;
use windows::{debug, experimental, frontend, graphics, input, log, paths, user};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum SettingsTab {
    #[default]
    General,
    Frontend,
    Graphics,
    User,
    Input,
    Paths,
    Log,
    Debug,
    Experimental,
}

impl SettingsTab {
    const ALL: [Self; 9] = [
        Self::General,
        Self::Frontend,
        Self::Graphics,
        Self::User,
        Self::Input,
        Self::Paths,
        Self::Log,
        Self::Debug,
        Self::Experimental,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Frontend => "Frontend",
            Self::Graphics => "Graphics",
            Self::User => "User",
            Self::Input => "Input",
            Self::Paths => "Paths",
            Self::Log => "Log",
            Self::Debug => "Debug",
            Self::Experimental => "Experimental",
        }
    }

    fn icon(self) -> egui::ImageSource<'static> {
        match self {
            Self::General => egui::include_image!("../../../assets/menubar/settings/general.svg"),
            Self::Frontend => egui::include_image!("../../../assets/menubar/settings/window.svg"),
            Self::Graphics => egui::include_image!("../../../assets/menubar/settings/graphics.svg"),
            Self::User => egui::include_image!("../../../assets/menubar/settings/general.svg"),
            Self::Input => egui::include_image!("../../../assets/menubar/settings/input.svg"),
            Self::Paths => egui::include_image!("../../../assets/menubar/settings/paths.svg"),
            Self::Log => egui::include_image!("../../../assets/menubar/settings/log.svg"),
            Self::Debug => egui::include_image!("../../../assets/menubar/settings/debug.svg"),
            Self::Experimental => {
                egui::include_image!("../../../assets/menubar/settings/experimental.svg")
            }
        }
    }

    fn show(
        self,
        ui: &mut egui::Ui,
        config: &mut Value,
        devices: &AudioDevices,
        gpu_devices: &mut GpuDevices,
        camera_devices: &mut CameraDevices,
    ) -> bool {
        match self {
            Self::General => return general::show(ui, config, devices),
            Self::Frontend => frontend::show(ui, config),
            Self::Graphics => graphics::show(ui, config, gpu_devices),
            Self::User => user::show(ui, config),
            Self::Input => input::show(ui, config, camera_devices),
            Self::Paths => paths::show(ui, config),
            Self::Log => log::show(ui, config),
            Self::Debug => debug::show(ui, config),
            Self::Experimental => experimental::show(ui, config),
        }
        false
    }
}

#[derive(Default)]
pub struct Settings {
    open: bool,
    selected_tab: SettingsTab,
    draft: Option<Value>,
    config_path: Option<PathBuf>,
    error: Option<String>,
    audio_devices: AudioDevices,
    gpu_devices: GpuDevices,
    camera_devices: CameraDevices,
}

impl Settings {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Settings", |ui| {
            if ui.button("Open settings").clicked() {
                if !self.open {
                    self.load();
                    if let Some(config) = &self.draft {
                        if let Some(home) = config["Frontend"]["home_tab"].as_str() {
                            if let Some(tab) =
                                SettingsTab::ALL.iter().find(|tab| tab.label() == home)
                            {
                                self.selected_tab = *tab;
                            }
                        }
                    }
                }
                self.open = true;
                ui.close();
            }
            ui.separator();
            for tab in SettingsTab::ALL {
                if ui
                    .add(
                        egui::Button::image_and_text(tab.icon(), tab.label())
                            .image_tint_follows_text_color(true),
                    )
                    .clicked()
                {
                    if !self.open {
                        self.load();
                    }
                    self.selected_tab = tab;
                    self.open = true;
                    ui.close();
                }
            }
        });
    }

    pub fn show_window(&mut self, ctx: &egui::Context) -> bool {
        self.camera_devices.poll();
        if !self.open {
            return false;
        }
        self.gpu_devices.ensure_scan(ctx);
        self.gpu_devices.poll();
        let mut open = self.open;
        let mut save = false;
        let mut close_after_save = false;
        let mut cancel = false;
        let mut refresh_devices = false;
        egui::Window::new("Settings")
            .id(egui::Id::new("settings_window"))
            .open(&mut open)
            .default_size([900.0, 600.0])
            .min_size([900.0, 600.0])
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for tab in SettingsTab::ALL {
                        let selected = self.selected_tab == tab;
                        if ui
                            .add(
                                egui::Button::image_and_text(tab.icon(), tab.label())
                                    .image_tint_follows_text_color(true)
                                    .selected(selected)
                                    .frame_when_inactive(selected),
                            )
                            .clicked()
                        {
                            self.selected_tab = tab;
                        }
                    }
                });
                ui.separator();
                if let Some(error) = &self.error {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }
                if let Some(config) = &mut self.draft {
                    egui::Panel::bottom("settings_actions")
                        .frame(egui::Frame::NONE)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                                if ui.button("Cancel").clicked() {
                                    cancel = true;
                                }
                                if ui.button("Apply").clicked() {
                                    save = true;
                                }
                                let selection = ui.visuals().selection;
                                let save_button = egui::Button::new(
                                    egui::RichText::new("Save").color(selection.stroke.color),
                                )
                                .fill(selection.bg_fill);
                                if ui.add(save_button).clicked() {
                                    save = true;
                                    close_after_save = true;
                                }
                            });
                        });
                    egui::ScrollArea::vertical()
                        .id_salt("settings_content")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            refresh_devices = self.selected_tab.show(
                                ui,
                                config,
                                &self.audio_devices,
                                &mut self.gpu_devices,
                                &mut self.camera_devices,
                            );
                        });
                } else if ui.button("Cancel").clicked() {
                    cancel = true;
                }
            });
        if let Some(config) = &self.draft {
            let backend = config["Audio"]["audio_backend"].as_u64().unwrap_or(0);
            if refresh_devices || backend != self.audio_devices.backend {
                self.audio_devices = AudioDevices::scan(backend);
                ctx.request_repaint();
            }
        }
        let saved = save && self.save();
        if saved && close_after_save {
            open = false;
        }
        if cancel {
            open = false;
        }
        self.open = open;
        if !open {
            self.draft = None;
            self.error = None;
        }
        saved
    }
}