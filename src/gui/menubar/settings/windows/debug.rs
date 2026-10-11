use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "General", |ui| {
                check(ui, &mut config.gpu.dump_shaders, "Enable Shaders Dumpng");
                check(ui, &mut config.debug.debug_dump, "Enable Debug Dumping");
                check(ui, &mut config.vulkan.renderdoc_enabled, "Enable RenderDoc Debugging");
                check(ui, &mut config.gpu.copy_gpu_buffers, "Copy GPU buffers");
                check(ui, &mut config.debug.shader_collect, "Collect shaders");
            });
            section(ui, "Vulkan Validation", |ui| {
                check(ui, &mut config.vulkan.vkvalidation_enabled, "Enable validation layers");
                if config.vulkan.vkvalidation_enabled {
                    section(ui, "Validation Layers", |ui| {
                        check(ui, &mut config.vulkan.vkvalidation_core_enabled, "Enable Core Validation");
                        check(ui, &mut config.vulkan.vkvalidation_sync_enabled, "Enable Sync Validation");
                        check(
                            ui,
                            &mut config.vulkan.vkvalidation_gpu_enabled,
                            "Enable GPU-Assisted Validation",
                        );
                    });
                }
            });
        },
        |ui, config| {
            section(ui, "Advanced", |ui| {
                check(ui, &mut config.debug.shader_collect, "Collect Shaders");
                check(ui, &mut config.vulkan.vkcrash_diagnostic_enabled, "Enable Crash Diagnostics");
                check(ui, &mut config.vulkan.vkguest_markers, "Guest Debug Markers");
                check(ui, &mut config.vulkan.vkhost_markers, "Host Debug Markers");
            });
        },
    );
}
