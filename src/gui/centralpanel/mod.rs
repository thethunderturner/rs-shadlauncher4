use eframe::egui;

#[derive(Default)]
pub struct Centralpanel {}

impl Centralpanel {
    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}
