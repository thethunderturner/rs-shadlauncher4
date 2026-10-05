use eframe::egui;
use eframe::egui::Button;

pub struct File {}

impl File {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            let _ = ui.add(
                Button::image_and_text(
                    egui::include_image!("../../../assets/menubar/file/install.svg"),
                    "Install Packages",
                )
                .image_tint_follows_text_color(true),
            );
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../../assets/menubar/file/exit.svg"),
                        "Exit",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
