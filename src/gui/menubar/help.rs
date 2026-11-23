use crate::gui::windows::about::AboutWindow;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Help {
    about_window: AboutWindow,
}

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            if ui
                .add(
                    Button::image_and_text(egui::include_image!("../../assets/info.svg"), "About")
                        .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                self.about_window.open = true
            }
        });
        if self.about_window.open {
            self.about_window.show(ui);
        }
    }
}
