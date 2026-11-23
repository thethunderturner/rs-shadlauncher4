use crate::config::emulator::Config;
use eframe::egui;

#[derive(Default)]
pub struct NetworkWindow;

impl NetworkWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Network Settings").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.network.connected_to_network, "Enable Network Connection");
                ui.checkbox(&mut config.network.shad_net_enabled, "Enable shadNet");
                ui.checkbox(&mut config.network.enable_upnp, "Enable UPnP");
                ui.checkbox(&mut config.network.disable_https, "Disable HTTPS");
            });
            columns[1].label(egui::RichText::new("shadNet Settings").strong());
            columns[1].group(|ui| {
                ui.label(egui::RichText::new("shadNet server").strong());
                ui.text_edit_singleline(&mut config.network.shadnet_server);
                ui.label(egui::RichText::new("Web API server").strong());
                ui.text_edit_singleline(&mut config.network.shadnet_webapi_server);
                ui.label(egui::RichText::new("P2P Port").strong());
                ui.add(egui::DragValue::new(&mut config.network.p2p_port).range(1024..=49151));
                ui.label(egui::RichText::new("Signaling info").strong());
                ui.text_edit_singleline(&mut config.network.signaling_info);
            });
        });
    }
}
