use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    ui.menu_button("Manage", |ui| {
        let _ = ui.button("User Accounts");
    });
}
