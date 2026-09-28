use eframe::egui;

pub struct Utilities;

impl Utilities {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Utilities", |ui| {
            let _ = ui.button("Setup Wizard");
            let _ = ui.button("Export Title List");
            let _ = ui.button("Reset All Custom Titles");
            let _ = ui.button("Crypto Key Manager");
            let _ = ui.button("Configure Hotkeys");
            let _ = ui.button("Network Host Override");
            let _ = ui.button("Manage Skylanders");
            let _ = ui.button("Manage Infinity Figures");
            let _ = ui.button("Manage Dimensions Toypad");
        });
    }
}
