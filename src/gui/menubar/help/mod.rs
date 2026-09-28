use eframe::egui;

pub fn show(ui: &mut egui::Ui, show_about: &mut bool) {
    ui.menu_button("Help", |ui| {
        let _ = ui.button("Check For Updates");
        if ui.button("About").clicked() {
            *show_about = true;
            ui.close();
        }
    });
}
