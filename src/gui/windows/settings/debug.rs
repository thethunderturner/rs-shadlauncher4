use crate::config::emulator::Config;
use eframe::egui;

#[derive(Default)]
pub struct DebugWindow;

impl DebugWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Logger").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.gpu.dump_shaders, "Enable Shaders Dumpng");
                ui.checkbox(&mut config.debug.debug_dump, "Enable Debug Dumping");
                ui.checkbox(&mut config.vulkan.renderdoc_enabled, "Enable RenderDoc Debugging");
                ui.checkbox(&mut config.gpu.copy_gpu_buffers, "Copy GPU buffers");
            });
            columns[0].label(egui::RichText::new("Vulkan Validation (Requires Vulkan SDK)").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.vulkan.vkvalidation_enabled, "Enable Vulkan Validation Layers");
                if config.vulkan.vkvalidation_enabled {
                    ui.group(|ui| {
                        ui.checkbox(&mut config.vulkan.vkvalidation_core_enabled, "Enable Core Validation");
                        ui.checkbox(&mut config.vulkan.vkvalidation_sync_enabled, "Enable Sync Validation");
                        ui.checkbox(
                            &mut config.vulkan.vkvalidation_gpu_enabled,
                            "Enable GPU-Assisted Validation",
                        );
                    });
                }
            });
            columns[1].label(egui::RichText::new("Advanced").strong());
            columns[1].group(|ui| {
                ui.checkbox(&mut config.debug.shader_collect, "Collect Shaders");
                ui.checkbox(&mut config.vulkan.vkcrash_diagnostic_enabled, "Enable Crash Diagnostics");
                ui.checkbox(&mut config.vulkan.vkguest_markers, "Guest Debug Markers");
                ui.checkbox(&mut config.vulkan.vkhost_markers, "Host Debug Markers");
            });
        });
    }
}
