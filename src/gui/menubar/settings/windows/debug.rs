use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Debugging", |ui| {
                check(ui, config, "Debug", "debug_dump", "Enable debug dumping");
                check(ui, config, "Debug", "shader_collect", "Collect shaders");
                check(ui, config, "GPU", "copy_gpu_buffers", "Copy GPU buffers");
            });
            section(ui, "RenderDoc", |ui| {
                check(
                    ui,
                    config,
                    "Vulkan",
                    "renderdoc_enabled",
                    "Enable RenderDoc debugging",
                );
            });
        },
        |ui, config| {
            section(ui, "Vulkan Validation", |ui| {
                check(
                    ui,
                    config,
                    "Vulkan",
                    "vkvalidation_enabled",
                    "Enable validation layers",
                );
                if config["Vulkan"]["vkvalidation_enabled"].as_bool() == Some(true) {
                    check(
                        ui,
                        config,
                        "Vulkan",
                        "vkvalidation_sync_enabled",
                        "Sync validation",
                    );
                    check(
                        ui,
                        config,
                        "Vulkan",
                        "vkvalidation_core_enabled",
                        "Core validation",
                    );
                    check(
                        ui,
                        config,
                        "Vulkan",
                        "vkvalidation_gpu_enabled",
                        "GPU assisted validation",
                    );
                }
            });
            section(ui, "Diagnostics and Markers", |ui| {
                check(
                    ui,
                    config,
                    "Vulkan",
                    "vkcrash_diagnostic_enabled",
                    "Crash diagnostics",
                );
                check(
                    ui,
                    config,
                    "Vulkan",
                    "vkguest_markers",
                    "Guest debug markers",
                );
                check(ui, config, "Vulkan", "vkhost_markers", "Host debug markers");
            });
        },
    );
}
