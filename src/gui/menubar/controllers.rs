use crate::gui::windows::controllers::ControllersWindow;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Controllers {
    window: ControllersWindow,
}

impl Controllers {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Controllers", |ui| {
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/controllers/gamepad.svg"),
                        "Manage controllers",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                self.window.open = true;
            };
        });

        if self.window.open {
            self.window.show(ui)
        }
    }
}
