use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Help;

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            ui.add(Button::image_and_text(
                egui::include_image!("../../assets/update.svg"),
                "Check For updates",
            ));
            ui.add(Button::image_and_text(
                egui::include_image!("../../assets/info.svg"),
                "About",
            ));
        });
    }
}
