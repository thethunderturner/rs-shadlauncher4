use eframe::egui;

#[derive(Default)]
pub struct Controllers;

impl Controllers {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Controllers", |ui| if ui.button("Manage controllers").clicked() {});
    }
}
