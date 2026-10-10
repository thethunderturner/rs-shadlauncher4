use crate::gui::menubar::settings::helpers::controls::*;
use crate::gui::menubar::settings::helpers::gpu_devices::GpuDevices;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(
    ui: &mut egui::Ui,
    config: &mut Value,
    gpu_devices: &mut GpuDevices,
) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Display Mode", |ui| {
                choice(
                    ui,
                    config,
                    "GPU",
                    "full_screen_mode",
                    "Mode",
                    &[
                        ("Windowed", "Windowed"),
                        ("Fullscreen (Borderless)", "Fullscreen (Borderless)"),
                        ("Fullscreen", "Fullscreen"),
                    ],
                );
                let fullscreen = matches!(
                    config["GPU"]["full_screen_mode"].as_str(),
                    Some("Fullscreen (Borderless)" | "Fullscreen")
                );
                config["GPU"]["full_screen"] = Value::Bool(fullscreen);
                choice(
                    ui,
                    config,
                    "GPU",
                    "present_mode",
                    "Present mode",
                    &[
                        ("Mailbox (Vsync)", "Mailbox"),
                        ("Fifo (Vsync)", "Fifo"),
                        ("Immediate (No Vsync)", "Immediate"),
                    ],
                );
            });
            section(ui, "Window Size", |ui| {
                uint(ui, config, "GPU", "window_width", "Width", 1..=16384);
                uint(ui, config, "GPU", "window_height", "Height", 1..=16384);
                uint(
                    ui,
                    config,
                    "GPU",
                    "vblank_frequency",
                    "Vblank frequency (Hz)",
                    1..=1000,
                );
            });
        },
        |ui, config| {
            section(ui, "Graphics Device", |ui| {
                graphics_device(ui, config, gpu_devices);
                check(ui, config, "GPU", "null_gpu", "Enable Null GPU");
            });
            section(ui, "Upscaling and Color", |ui| {
                check(ui, config, "GPU", "hdr_allowed", "Enable HDR");
                check(ui, config, "GPU", "fsr_enabled", "Enable FSR");
                check(ui, config, "GPU", "rcas_enabled", "Enable RCAS sharpening");
                uint(
                    ui,
                    config,
                    "GPU",
                    "rcas_attenuation",
                    "RCAS attenuation",
                    0..=1000,
                );
            });
        },
    );
}

fn graphics_device(ui: &mut egui::Ui, config: &mut Value, devices: &mut GpuDevices) {
    let mut selected = config["Vulkan"]["gpu_id"].as_i64().unwrap_or(-1);
    let selected_name = if selected == -1 {
        "Auto Select".to_owned()
    } else {
        usize::try_from(selected)
            .ok()
            .and_then(|index| devices.names().get(index))
            .cloned()
            .unwrap_or_else(|| format!("Unavailable GPU (ID {selected})"))
    };
    ui.horizontal(|ui| {
        ui.label("Device");
        egui::ComboBox::from_id_salt("graphics_device")
            .selected_text(selected_name)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, -1, "Auto Select");
                for (index, name) in devices.names().iter().enumerate() {
                    ui.selectable_value(&mut selected, index as i64, name);
                }
            });
    });
    if devices.is_scanning() {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Finding graphics devices…");
        });
    } else if let Some(error) = devices.error() {
        ui.colored_label(ui.visuals().error_fg_color, error);
    } else if devices.names().is_empty() {
        ui.label("No graphics devices found.");
    }
    if ui
        .add_enabled(!devices.is_scanning(), egui::Button::new("Refresh devices"))
        .clicked()
    {
        devices.start_scan(ui.ctx());
    }
    config["Vulkan"]["gpu_id"] = Value::from(selected);
}
