use crate::gui::menubar::settings::helpers::audio_devices::AudioDevices;
use crate::gui::menubar::settings::helpers::controls::{check, section, two_columns, uint};
use eframe::egui;
use serde_json::Value;

const LANGUAGES: &[(&str, u64)] = &[
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

pub fn show(ui: &mut egui::Ui, config: &mut Value, devices: &AudioDevices) -> bool {
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
                    let value = &mut config["General"]["console_language"];
                    let mut current = value.as_u64().unwrap_or(1);
                    egui::ComboBox::from_id_salt("console_language")
                        .width(ui.available_width())
                        .selected_text(
                            LANGUAGES
                                .iter()
                                .find(|(_, id)| *id == current)
                                .map_or("Unknown", |(name, _)| name),
                        )
                        .show_ui(ui, |ui| {
                            for (name, id) in LANGUAGES {
                                ui.selectable_value(&mut current, *id, *name);
                            }
                        });
                    *value = Value::from(current);
                });
                section(ui, "Miscellaneous", |ui| {
                    check(
                        ui,
                        config,
                        "General",
                        "show_splash",
                        "Show Splash Screen When Launching Game",
                    );
                    check(
                        ui,
                        config,
                        "General",
                        "discord_rpc_enabled",
                        "Discord Rich Presence",
                    );
                });
                section(ui, "Volume", |ui| {
                    let value = &mut config["General"]["volume_slider"];
                    let mut volume = value.as_u64().unwrap_or(100);
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [48.0, ui.spacing().interact_size.y],
                            egui::Label::new(format!("{volume}%")),
                        );
                        ui.spacing_mut().slider_width = ui.available_width().max(100.0);
                        if ui
                            .add(egui::Slider::new(&mut volume, 0..=500).show_value(false))
                            .changed()
                        {
                            *value = Value::from(volume);
                        }
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
                    let value = &mut config["Audio"]["audio_backend"];
                    let mut current = value.as_u64().unwrap_or(0);
                    egui::ComboBox::from_id_salt("audio_backend")
                        .width(ui.available_width())
                        .selected_text(if current == 1 { "OpenAL" } else { "SDL" })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut current, 0, "SDL");
                            ui.selectable_value(&mut current, 1, "OpenAL");
                        });
                    *value = Value::from(current);
                });

                let backend = config["Audio"]["audio_backend"].as_u64().unwrap_or(0);
                let prefix = if backend == 1 { "openal" } else { "sdl" };
                device(
                    ui,
                    config,
                    prefix,
                    "main_output_device",
                    "Audio Device (general)",
                    &devices.outputs,
                );
                device(
                    ui,
                    config,
                    prefix,
                    "pad_spk_output_device",
                    "Audio Device (DS4 speaker)",
                    &devices.outputs,
                );
                device(
                    ui,
                    config,
                    prefix,
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
                        uint(ui, config, "Audio", "openal_hrtf", "HRTF", 0..=2);
                        uint(
                            ui,
                            config,
                            "Audio",
                            "openal_output_mode",
                            "Output mode",
                            0..=2,
                        );
                    });
                }
            });
        },
    );
    refresh
}

fn device(
    ui: &mut egui::Ui,
    config: &mut Value,
    prefix: &str,
    suffix: &str,
    label: &str,
    devices: &[String],
) {
    let key = format!("{prefix}_{suffix}");
    section(ui, label, |ui| {
        let value = &mut config["Audio"][key.as_str()];
        let mut current = value.as_str().unwrap_or("Default Device").to_owned();
        if current.is_empty() {
            current = "Default Device".to_owned();
        }
        let selected = if current == "Default Device" || devices.iter().any(|name| name == &current)
        {
            current.clone()
        } else {
            format!("{current} (unavailable)")
        };
        egui::ComboBox::from_id_salt(("audio_device", key.as_str()))
            .width(ui.available_width())
            .selected_text(selected)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current, "Default Device".to_owned(), "Default Device");
                for name in devices {
                    ui.selectable_value(&mut current, name.clone(), name);
                }
            });
        *value = Value::String(current);
    });
}
