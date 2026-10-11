use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Debugging", |ui| {
                check(ui, &mut config.debug.debug_dump, "Enable debug dumping");
                check(ui, &mut config.debug.shader_collect, "Collect shaders");
                check(ui, &mut config.gpu.copy_gpu_buffers, "Copy GPU buffers");
            });
            section(ui, "RenderDoc", |ui| {
                check(
                    ui,
                    &mut config.vulkan.renderdoc_enabled,
                    "Enable RenderDoc debugging",
                );
            });
        },
        |ui, config| {
            section(ui, "Vulkan Validation", |ui| {
                check(
                    ui,
                    &mut config.vulkan.vkvalidation_enabled,
                    "Enable validation layers",
                );
                if config.vulkan.vkvalidation_enabled {
                    check(
                        ui,
                        &mut config.vulkan.vkvalidation_sync_enabled,
                        "Sync validation",
                    );
                    check(
                        ui,
                        &mut config.vulkan.vkvalidation_core_enabled,
                        "Core validation",
                    );
                    check(
                        ui,
                        &mut config.vulkan.vkvalidation_gpu_enabled,
                        "GPU assisted validation",
                    );
                }
            });
            section(ui, "Diagnostics and Markers", |ui| {
                check(
                    ui,
                    &mut config.vulkan.vkcrash_diagnostic_enabled,
                    "Crash diagnostics",
                );
                check(
                    ui,
                    &mut config.vulkan.vkguest_markers,
                    "Guest debug markers",
                );
                check(ui, &mut config.vulkan.vkhost_markers, "Host debug markers");
            });
        },
    );
}
