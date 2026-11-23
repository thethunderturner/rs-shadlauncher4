use eframe::egui;

#[derive(Default)]
pub struct Help;

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            let _ = ui.button("Check For Updates");
            let _ = ui.button("About");
        });
    }
}
