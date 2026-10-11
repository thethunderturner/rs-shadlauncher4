use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;
use serde_json::Value;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Value) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Network", |ui| {
                check(
                    ui,
                    config,
                    "Network",
                    "connected_to_network",
                    "Enable Network Connection",
                );
                check(ui, config, "Network", "shad_net_enabled", "Enable shadNet");
                check(ui, config, "Network", "enable_upnp", "Enable UPnP");
                check(ui, config, "Network", "disable_https", "Disable HTTPS");
            });
        },
        |ui, config| {
            section(ui, "shadNet", |ui| {
                text(ui, config, "Network", "shadnet_server", "shadNet server");
                text(
                    ui,
                    config,
                    "Network",
                    "shadnet_webapi_server",
                    "Web API server",
                );
                uint(
                    ui,
                    config,
                    "Network",
                    "p2p_port",
                    "P2P Port",
                    1024..=49151,
                );
                text(ui, config, "Network", "signaling_info", "Signaling info");
            })
        },
    );
}
