use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
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
                int(
                    ui,
                    config,
                    "Vulkan",
                    "gpu_id",
                    "Device ID (-1: auto)",
                    -1..=64,
                );
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
            section(ui, "Diagnostics", |ui| {
                check(ui, config, "GPU", "null_gpu", "Enable Null GPU");
            });
        },
    );
}
