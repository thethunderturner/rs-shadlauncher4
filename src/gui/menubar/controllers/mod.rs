use eframe::egui;

pub(super) fn show(ui: &mut egui::Ui, show_controllers: &mut bool) {
    if ui.button("Controllers").clicked() {
        *show_controllers = true;
    }
}

pub(crate) fn show_window(ctx: &egui::Context, show_controllers: &mut bool) {
    if *show_controllers {
        egui::Window::new("Controllers")
            .open(show_controllers)
            .default_size([320.0, 240.0])
            .show(ctx, |ui| {
                ui.label("Controller settings will appear here.");
            });
    }
}
