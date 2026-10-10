use eframe::egui;

#[derive(Default)]
pub struct Toolbar {}

impl Toolbar {
    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Panel::top("games_folder")
            .resizable(false)
            .show(ui, |ui| {
                ui.add(egui::Label::new(format!("Versions (TBA)")).truncate())
            });
    }
}
