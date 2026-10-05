use eframe::egui;
use eframe::egui::Button;

pub struct Emulation {}

impl Emulation {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Emulation", |ui| {
            let _ = ui.add(
                Button::image_and_text(
                    egui::include_image!("../../../assets/menubar/emulation/play.svg"),
                    "Run",
                )
                .image_tint_follows_text_color(true),
            );
            let _ = ui.add(
                Button::image_and_text(
                    egui::include_image!("../../../assets/menubar/emulation/pause.svg"),
                    "Pause",
                )
                .image_tint_follows_text_color(true),
            );
            let _ = ui.add(
                Button::image_and_text(
                    egui::include_image!("../../../assets/menubar/emulation/stop.svg"),
                    "Stop",
                )
                .image_tint_follows_text_color(true),
            );
            let _ = ui.add(
                Button::image_and_text(
                    egui::include_image!("../../../assets/menubar/emulation/restart.svg"),
                    "Restart",
                )
                .image_tint_follows_text_color(true),
            );
        });
    }
}
