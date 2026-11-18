use crate::config::emulator::Config;
use crate::devices::audio::AudioDevices;
use eframe::egui;

#[derive(Default)]
pub struct GeneralWindow {
    audio_devices: AudioDevices,
}

const LANGUAGES: &[(&str, u32)] = &[
    ("Arabic", 21),
    ("Czech", 23),
    ("Danish", 14),
    ("Dutch", 6),
    ("English (United Kingdom)", 18),
    ("English (United States)", 1),
    ("Finnish", 12),
    ("French (Canada)", 22),
    ("French (France)", 2),
    ("German", 4),
    ("Greek", 25),
    ("Hungarian", 24),
    ("Indonesian", 29),
    ("Italian", 5),
    ("Japanese", 0),
    ("Korean", 9),
    ("Norwegian (Bokmaal)", 15),
    ("Polish", 16),
    ("Portuguese (Brazil)", 17),
    ("Portuguese (Portugal)", 7),
    ("Romanian", 26),
    ("Russian", 8),
    ("Simplified Chinese", 11),
    ("Spanish (Latin America)", 20),
    ("Spanish (Spain)", 3),
    ("Swedish", 13),
    ("Thai", 27),
    ("Traditional Chinese", 10),
    ("Turkish", 19),
    ("Ukrainian", 30),
    ("Vietnamese", 28),
];

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
                            LANGUAGES
                                .iter()
                                .find(|(_, id)| id == current)
                                .map_or("Unknown", |(name, _)| name),
                        )
                        .show_ui(ui, |ui| {
                            for (name, id) in LANGUAGES {
                                ui.selectable_value(current, *id, *name);
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
