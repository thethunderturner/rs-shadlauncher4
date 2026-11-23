use eframe::egui;
use eframe::egui::Button;
use crate::gui::windows::manage::ManageWindow;

#[derive(Default)]
pub struct Manage {
    window: ManageWindow,
}

impl Manage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Manage", |ui| {
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/user-group.svg"),
                        "Manage Users",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                self.window.open = true
            }
        });
        if self.window.open {
            self.window.show(ui)
        }
    }
}