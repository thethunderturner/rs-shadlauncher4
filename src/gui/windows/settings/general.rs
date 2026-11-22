use crate::config::emulator::Config;
use crate::devices::audio::AudioDevices;
use crate::localization::LANGUAGE;
use eframe::egui;

#[derive(Default)]
pub struct GeneralWindow {
    audio_devices: AudioDevices,
}

impl GeneralWindow {
    pub fn new(backend: u32) -> Self {
        Self {
            audio_devices: AudioDevices::scan(backend),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("System").strong());
            columns[0].group(|ui| {
                ui.label(egui::RichText::new("Console Language").strong());
                ui.group(|ui| {
                    let current = &mut config.general.console_language;
                    egui::ComboBox::from_id_salt("console_language")
                        .width(ui.available_width())
                        .selected_text(
                            LANGUAGE::from_id(*current).map_or_else(|| "Unknown".to_owned(), LANGUAGE::label),
                        )
                        .show_ui(ui, |ui| {
                            for language in LANGUAGE::ALL {
                                ui.selectable_value(current, *language as u32, LANGUAGE::label(*language));
                            }
                        });
                });
                ui.label(egui::RichText::new("Miscellaneous").strong());
                ui.group(|ui| {
                    ui.checkbox(&mut config.general.show_splash, "Show Splash Screen When Launching Game");
                    ui.checkbox(&mut config.general.discord_rpc_enabled, "Discord Rich Presence");
                });
                ui.label(egui::RichText::new("Volume").strong());
                ui.group(|ui| {
                    ui.add(egui::Slider::new(&mut config.general.volume_slider, 0..=500));
                });
            });

            columns[1].label(egui::RichText::new("Audio").strong());
            columns[1].group(|ui| {
                ui.label(egui::RichText::new("Audio Backend").strong());
                ui.group(|ui| {
                    let current = &mut config.audio.audio_backend;
                    egui::ComboBox::from_id_salt("audio_backend")
                        .width(ui.available_width())
                        .selected_text(if *current == 1 { "OpenAL" } else { "SDL" })
                        .show_ui(ui, |ui| {
                            if ui.selectable_value(current, 0, "SDL").clicked() {
                                self.audio_devices = AudioDevices::scan(0);
                            }
                            if ui.selectable_value(current, 1, "OpenAL").clicked() {
                                self.audio_devices = AudioDevices::scan(1);
                            }
                        });
                });

                let backend = config.audio.audio_backend;
                ui.label(egui::RichText::new("Audio Device (General)").strong());
                ui.group(|ui| {
                    let current = match backend {
                        0 => &mut config.audio.sdl_main_output_device,
                        1 => &mut config.audio.openal_main_output_device,
                        _ => return,
                    };
                    egui::ComboBox::from_id_salt("audio_device_general")
                        .width(ui.available_width())
                        .selected_text(current.as_str())
                        .show_ui(ui, |ui| {
                            for device in &self.audio_devices.outputs {
                                ui.selectable_value(current, device.clone(), device.as_str());
                            }
                        });
                });

                ui.label(egui::RichText::new("Audio Device (DS4 speaker)").strong());
                ui.group(|ui| {
                    let current = match backend {
                        0 => &mut config.audio.sdl_pad_spk_output_device,
                        1 => &mut config.audio.openal_pad_spk_output_device,
                        _ => return,
                    };
                    egui::ComboBox::from_id_salt("audio_device_pad_spk")
                        .width(ui.available_width())
                        .selected_text(current.as_str())
                        .show_ui(ui, |ui| {
                            for device in &self.audio_devices.outputs {
                                ui.selectable_value(current, device.clone(), device.as_str());
                            }
                        });
                });

                ui.label(egui::RichText::new("Input Device").strong());
                ui.group(|ui| {
                    let current = match backend {
                        0 => &mut config.audio.sdl_mic_device,
                        1 => &mut config.audio.openal_mic_device,
                        _ => return,
                    };
                    egui::ComboBox::from_id_salt("input_device")
                        .width(ui.available_width())
                        .selected_text(current.as_str())
                        .show_ui(ui, |ui| {
                            for device in &self.audio_devices.inputs {
                                ui.selectable_value(current, device.clone(), device.as_str());
                            }
                        });
                });
            });
        });
    }
}
