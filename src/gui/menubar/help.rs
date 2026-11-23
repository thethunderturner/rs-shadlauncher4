use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Help;

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            ui.add(
                Button::image_and_text(egui::include_image!("../../assets/update.svg"), "Check For updates")
                    .image_tint_follows_text_color(true),
            );
            ui.add(
                Button::image_and_text(egui::include_image!("../../assets/info.svg"), "About")
                    .image_tint_follows_text_color(true),
            );
        });
    }
}
