use eframe::egui;

pub(super) fn show(ui: &mut egui::Ui) {
    egui::Panel::left("sidebar")
        .default_size(150.0)
        .max_size(250.0)
        .resizable(true)
        .show(ui, |ui| {
            ui.heading("Library");
            ui.label("All games:  TBD");
            ui.label("Favorites:  TBD");
            ui.separator();
            ui.horizontal(|ui| {
                ui.strong("Folders");
                if ui.small_button("+").on_hover_text("New folder").clicked() {
                    // TODO
                }
            });
        });
}
