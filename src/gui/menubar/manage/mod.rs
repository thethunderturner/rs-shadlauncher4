use eframe::egui;

pub struct Manage {}

impl Manage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Manage", |ui| {
            let _ = ui.button("User Accounts");
        });
    }
}
