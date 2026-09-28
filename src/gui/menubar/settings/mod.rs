use eframe::egui;
pub fn show(ui: &mut egui::Ui) {
    ui.menu_button("Settings", |ui| {
        let _ = ui.button("General");
        let _ = ui.button("Audio");
        let _ = ui.button("GUI");
        let _ = ui.button("Graphics");
        let _ = ui.button("Input");
        let _ = ui.button("Paths");
        let _ = ui.button("Log");
        let _ = ui.button("Debug");
        let _ = ui.button("Experimental");
    });
}
