use eframe::egui;

pub struct Emulation {}

impl Emulation {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Emulation", |ui| {
            let _ = ui.button("Run");
            let _ = ui.button("Pause");
            let _ = ui.button("Stop");
            let _ = ui.button("Restart");
        });
    }
}
