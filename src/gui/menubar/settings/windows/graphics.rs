use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use crate::gui::menubar::settings::helpers::gpu_devices::GpuDevices;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(
    ui: &mut egui::Ui,
    config: &mut Config,
    gpu_devices: &mut GpuDevices,
) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Display Mode", |ui| {
                choice(
                    ui,
                    &mut config.gpu.full_screen_mode,
                    "full_screen_mode",
                    "Mode",
                    &[
                        ("Windowed", "Windowed"),
                        ("Fullscreen (Borderless)", "Fullscreen (Borderless)"),
                        ("Fullscreen", "Fullscreen"),
                    ],
                );
                let fullscreen = matches!(
                    config.gpu.full_screen_mode.as_str(),
                    "Fullscreen (Borderless)" | "Fullscreen"
                );
                config.gpu.full_screen = fullscreen;
                choice(
                    ui,
                    &mut config.gpu.present_mode,
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
                uint(ui, &mut config.gpu.window_width, "Width", 1..=16384);
                uint(ui, &mut config.gpu.window_height, "Height", 1..=16384);
                uint(
                    ui,
                    &mut config.gpu.vblank_frequency,
                    "Vblank frequency (Hz)",
                    1..=1000,
                );
            });
        },
        |ui, config| {
            section(ui, "Graphics Device", |ui| {
                graphics_device(ui, config, gpu_devices);
                check(ui, &mut config.gpu.null_gpu, "Enable Null GPU");
            });
            section(ui, "Upscaling and Color", |ui| {
                check(ui, &mut config.gpu.hdr_allowed, "Enable HDR");
                check(ui, &mut config.gpu.fsr_enabled, "Enable FSR");
                check(ui, &mut config.gpu.rcas_enabled, "Enable RCAS sharpening");
                rcas_attenuation(ui, config);
            });
        },
    );
}

fn rcas_attenuation(ui: &mut egui::Ui, config: &mut Config) {
    let value = &mut config.gpu.rcas_attenuation;
    ui.label("RCAS attenuation");
    ui.horizontal(|ui| {
        ui.add(egui::Slider::new(value, 0..=3000).show_value(false));
        ui.label(format!("{:.3}", *value as f64 / 1000.0));
    });
}

fn graphics_device(ui: &mut egui::Ui, config: &mut Config, devices: &mut GpuDevices) {
    let selected = &mut config.vulkan.gpu_id;
    let selected_name = if *selected == -1 {
        "Auto Select".to_owned()
    } else {
        usize::try_from(*selected)
            .ok()
            .and_then(|index| devices.names().get(index))
            .cloned()
            .unwrap_or_else(|| format!("Unavailable GPU (ID {selected})"))
    };
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("graphics_device")
            .selected_text(selected_name)
            .show_ui(ui, |ui| {
                ui.selectable_value(selected, -1, "Auto Select");
                for (index, name) in devices.names().iter().enumerate() {
                    ui.selectable_value(selected, index as i32, name);
                }
            });
        if devices.is_scanning() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Searching");
            });
        } else if let Some(error) = devices.error() {
            ui.colored_label(ui.visuals().error_fg_color, error);
        } else if devices.names().is_empty() {
            ui.label("No graphics devices found.");
        }
    });
}
