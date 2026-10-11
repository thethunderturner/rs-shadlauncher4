use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Graphics and Memory", |ui| {
                index(
                    ui,
                    &mut config.gpu.readbacks_mode,
                    "readbacks_mode",
                    "Readbacks mode",
                    &["Disabled", "Enabled", "Hybrid"],
                );
                check(
                    ui,
                    &mut config.gpu.readback_linear_images_enabled,
                    "Read back linear images",
                );
                check(
                    ui,
                    &mut config.gpu.direct_memory_access_enabled,
                    "Direct memory access",
                );
                check(ui, &mut config.gpu.dump_shaders, "Dump shaders");
                uint(
                    ui,
                    &mut config.general.extra_dmem_in_mbytes,
                    "Additional DMem (MiB)",
                    0..=65536,
                );
            });
            section(ui, "Shader Cache", |ui| {
                check(
                    ui,
                    &mut config.vulkan.pipeline_cache_enabled,
                    "Enable shader cache",
                );
                if config.vulkan.pipeline_cache_enabled {
                    check(
                        ui,
                        &mut config.vulkan.pipeline_cache_archived,
                        "Compress shader cache",
                    );
                }
            });
        },
        |ui, config| {
            section(ui, "Console", |ui| {
                check(ui, &mut config.general.neo_mode, "PS4 Neo mode");
                check(ui, &mut config.general.dev_kit_mode, "Devkit mode");
            });
        },
    );
}
