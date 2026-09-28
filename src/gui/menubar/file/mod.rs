use eframe::egui;

pub struct File {}

impl File {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            let _ = ui.button("Install Packages");
            if ui.button("Exit").clicked() {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
