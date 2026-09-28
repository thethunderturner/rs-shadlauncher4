use eframe::egui;

pub struct Controllers {
    pub show_controllers: bool,
}

impl Controllers {
    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        if ui.button("Controllers").clicked() {
            self.show_controllers = true;
        }
    }

    pub(super) fn show_window(&mut self, ctx: &egui::Context) {
        if self.show_controllers {
            egui::Window::new("Controllers")
                .open(&mut self.show_controllers)
                .default_size([320.0, 240.0])
                .show(ctx, |ui| {
                    ui.label("Controller settings will appear here.");
                });
        }
    }
}
