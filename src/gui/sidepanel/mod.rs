use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Sidepanel {}

impl Sidepanel {
    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("sidebar")
            .default_size(170.0)
            .max_size(250.0)
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("Search");
                // TODO search here

                ui.separator();

                ui.heading("Library");
                ui.label("All games:  TBD");
                ui.label("Favorites:  TBD");
                ui.label("Demos:  TBD");
                ui.label("VR:  TBD");
                ui.label("International:  TBD");
                ui.horizontal(|ui| {
                    ui.strong("Folders");
                    if ui.small_button("+").on_hover_text("New folder").clicked() {
                        // TODO
                    }
                });

                ui.separator();

                ui.heading("Refresh");
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/refresh/refresh-titles.svg"),
                        "Refresh List",
                    )
                        .image_tint_follows_text_color(true),
                );
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/refresh/refresh-compat.svg"),
                        "Refresh Compatibility",
                    )
                        .image_tint_follows_text_color(true),
                );

                ui.separator();

                ui.heading("Emulation");
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/emulation/play.svg"),
                        "Run",
                    )
                        .image_tint_follows_text_color(true),
                );
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/emulation/pause.svg"),
                        "Pause",
                    )
                        .image_tint_follows_text_color(true),
                );
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/emulation/stop.svg"),
                        "Stop",
                    )
                        .image_tint_follows_text_color(true),
                );
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/centralpanel/emulation/restart.svg"),
                        "Restart",
                    )
                        .image_tint_follows_text_color(true),
                );

            });
    }
}
