use crate::common::config::Config;
use crate::gui::menubar::settings::helpers::controls::*;
use eframe::egui;

pub(in crate::gui::menubar::settings) fn show(ui: &mut egui::Ui, config: &mut Config) {
    two_columns(
        ui,
        config,
        |ui, config| {
            section(ui, "Network", |ui| {
                check(ui, &mut config.network.connected_to_network, "Enable Network Connection");
                check(ui, &mut config.network.shad_net_enabled, "Enable shadNet");
                check(ui, &mut config.network.enable_upnp, "Enable UPnP");
                check(ui, &mut config.network.disable_https, "Disable HTTPS");
            });
        },
        |ui, config| {
            section(ui, "shadNet", |ui| {
                text(ui, &mut config.network.shadnet_server, "shadNet server");
                text(ui, &mut config.network.shadnet_webapi_server, "Web API server");
                uint(ui, &mut config.network.p2p_port, "P2P Port", 1024..=49151);
                text(ui, &mut config.network.signaling_info, "Signaling info");
            })
        },
    );
}
