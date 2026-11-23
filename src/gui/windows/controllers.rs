use eframe::egui;

#[derive(Default)]
pub struct ControllersWindow {
    pub(crate) open: bool,
}

impl ControllersWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Manage Controllers")
            .open(&mut self.open)
            .default_size(egui::vec2(500.0, 200.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Coming soon");
            });
    }
}

