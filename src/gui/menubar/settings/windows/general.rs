use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::audio_devices::AudioDevices;
use crate::gui::menubar::settings::helpers::controls::{check, section, two_columns, uint};
use eframe::egui;

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

pub fn show(ui: &mut egui::Ui, config: &mut Config, devices: &AudioDevices) -> bool {
    let mut refresh = false;
    two_columns(
        ui,
        config,
        |ui, config| {
            ui.label(egui::RichText::new("System").strong());
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.set_min_height(280.0);
                section(ui, "Console Language", |ui| {
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
                section(ui, "Miscellaneous", |ui| {
                    check(ui, &mut config.general.show_splash, "Show Splash Screen When Launching Game");
                    check(ui, &mut config.general.discord_rpc_enabled, "Discord Rich Presence");
                });
                section(ui, "Volume", |ui| {
                    let volume = &mut config.general.volume_slider;
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [48.0, ui.spacing().interact_size.y],
                            egui::Label::new(format!("{volume}%")),
                        );
                        ui.spacing_mut().slider_width = ui.available_width().max(100.0);
                        ui.add(egui::Slider::new(volume, 0..=500).show_value(false));
                    });
                });
            });
        },
        |ui, config| {
            ui.label(egui::RichText::new("Audio").strong());
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.set_min_height(280.0);
                section(ui, "Audio Backend", |ui| {
                    let current = &mut config.audio.audio_backend;
                    egui::ComboBox::from_id_salt("audio_backend")
                        .width(ui.available_width())
                        .selected_text(if *current == 1 { "OpenAL" } else { "SDL" })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(current, 0, "SDL");
                            ui.selectable_value(current, 1, "OpenAL");
                        });
                });

                let backend = config.audio.audio_backend;

                device(
                    ui,
                    if backend == 1 {
                        &mut config.audio.openal_main_output_device
                    } else {
                        &mut config.audio.sdl_main_output_device
                    },
                    "main_output_device",
                    "Audio Device (general)",
                    &devices.outputs,
                );
                device(
                    ui,
                    if backend == 1 {
                        &mut config.audio.openal_pad_spk_output_device
                    } else {
                        &mut config.audio.sdl_pad_spk_output_device
                    },
                    "pad_spk_output_device",
                    "Audio Device (DS4 speaker)",
                    &devices.outputs,
                );
                device(
                    ui,
                    if backend == 1 {
                        &mut config.audio.openal_mic_device
                    } else {
                        &mut config.audio.sdl_mic_device
                    },
                    "mic_device",
                    "Mic Device",
                    &devices.inputs,
                );
                if ui.button("Refresh audio devices").clicked() {
                    refresh = true;
                }
                if let Some(error) = &devices.error {
                    ui.colored_label(ui.visuals().warn_fg_color, error);
                }
                if backend == 1 {
                    section(ui, "OpenAL", |ui| {
                        uint(ui, &mut config.audio.openal_hrtf, "HRTF", 0..=2);
                        uint(ui, &mut config.audio.openal_output_mode, "Output mode", 0..=2);
                    });
                }
            });
        },
    );
    refresh
}

fn device(ui: &mut egui::Ui, value: &mut String, id: &str, label: &str, devices: &[String]) {
    section(ui, label, |ui| {
        let mut current = value.clone();
        if current.is_empty() {
            current = "Default Device".to_owned();
        }
        let selected = if current == "Default Device" || devices.iter().any(|name| name == &current) {
            current.clone()
        } else {
            format!("{current} (unavailable)")
        };
        egui::ComboBox::from_id_salt(("audio_device", id))
            .width(ui.available_width())
            .selected_text(selected)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current, "Default Device".to_owned(), "Default Device");
                for name in devices {
                    ui.selectable_value(&mut current, name.clone(), name);
                }
            });
        *value = current;
    });
}
