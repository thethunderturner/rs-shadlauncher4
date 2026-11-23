use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Manage {
    window: ManageWindow,
}

#[derive(Default)]
pub struct ManageWindow {
    open: bool,
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

impl ManageWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Manage Users")
            .open(&mut self.open)
            .default_size(egui::vec2(500.0, 200.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Coming soon");
            });
    }
}
