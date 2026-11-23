use crate::gui::windows::about::AboutWindow;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Help {
    about_window: AboutWindow,
    update_window: UpdateWindow,
}

#[derive(Default)]
pub struct UpdateWindow {
    open: bool,
}

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/update.svg"),
                        "Check for updates",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                self.update_window.open = true
            }
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
        if self.update_window.open {
            self.update_window.show(ui);
        }
    }
}

impl UpdateWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Update Launcher")
            .open(&mut self.open)
            .default_size(egui::vec2(500.0, 200.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Coming soon");
            });
    }
}
