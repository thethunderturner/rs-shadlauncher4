use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Graphics and Memory", |ui| {
                index(
                    ui,
                    config,
                    "GPU",
                    "readbacks_mode",
                    "Readbacks mode",
                    &["Disabled", "Enabled", "Hybrid"],
                );
                check(
                    ui,
                    config,
                    "GPU",
                    "readback_linear_images_enabled",
                    "Read back linear images",
                );
                check(
                    ui,
                    config,
                    "GPU",
                    "direct_memory_access_enabled",
                    "Direct memory access",
                );
                check(ui, config, "GPU", "dump_shaders", "Dump shaders");
                uint(
                    ui,
                    config,
                    "General",
                    "extra_dmem_in_mbytes",
                    "Additional DMem (MiB)",
                    0..=65536,
                );
            });
            section(ui, "Shader Cache", |ui| {
                check(
                    ui,
                    config,
                    "Vulkan",
                    "pipeline_cache_enabled",
                    "Enable shader cache",
                );
                if config["Vulkan"]["pipeline_cache_enabled"].as_bool() == Some(true) {
                    check(
                        ui,
                        config,
                        "Vulkan",
                        "pipeline_cache_archived",
                        "Compress shader cache",
                    );
                }
            });
        },
        |ui, config| {
            section(ui, "Console", |ui| {
                check(ui, config, "General", "neo_mode", "PS4 Neo mode");
                check(ui, config, "General", "dev_kit_mode", "Devkit mode");
            });
            section(ui, "Network", |ui| {
                check(
                    ui,
                    config,
                    "Network",
                    "connected_to_network",
                    "Network connected",
                );
                check(ui, config, "Network", "enable_upnp", "Enable UPnP");
                check(ui, config, "Network", "shad_net_enabled", "Enable shadNet");
                text(ui, config, "Network", "shadnet_server", "shadNet server");
                text(
                    ui,
                    config,
                    "Network",
                    "shadnet_webapi_server",
                    "Web API server",
                );
                text(ui, config, "Network", "signaling_info", "Signaling info");
            });
        },
    );
}
