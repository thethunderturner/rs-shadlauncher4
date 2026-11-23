use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Controllers {
    window: ControllersWindow,
}
#[derive(Default)]
pub struct ControllersWindow {
    open: bool,
}

impl Controllers {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Controllers", |ui| {
            if ui
                .add(Button::image_and_text(
                    egui::include_image!("../../assets/menubar/controllers/gamepad.svg"),
                    "Manage controllers",
                ))
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

impl ControllersWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Manage Controllers")
            .open(&mut self.open)
            .default_size(egui::vec2(500.0, 200.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Coming soon");
            });
    }
}
