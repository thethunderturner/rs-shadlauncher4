use eframe::egui;

#[derive(Default)]
pub struct ManageWindow {
    pub open: bool,
}

impl ManageWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Manage Users")
            .open(&mut self.open)
            .default_size(egui::vec2(500.0, 200.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Coming soon");
            });
    }
}
