pub mod actions;

use crate::gui::sidepanel::actions::SidepanelActions;
use crate::gui::sidepanel::actions::search::Search;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Sidepanel {
    search: Search,
}

impl Sidepanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        scanning: bool,
        can_refresh: bool,
    ) -> SidepanelActions<'_> {
        let mut search_changed = false;
        let mut refresh_list = false;
        egui::Panel::left("sidebar")
            .default_size(175.0)
            .max_size(250.0)
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("Search");
                search_changed = ui
                    .add(
                        egui::TextEdit::singleline(&mut self.search.query)
                            .id_salt("game_search")
                            .hint_text("Title or CUSA")
                            .desired_width(f32::INFINITY),
                    )
                    .changed();

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

                ui.separator();

                ui.heading("Refresh");
                refresh_list = ui
                    .add_enabled(
                        !scanning && can_refresh,
                        Button::image_and_text(
                            egui::include_image!(
                                "../../assets/centralpanel/refresh/refresh-titles.svg"
                            ),
                            if scanning { "Scanning" } else { "Refresh List" },
                        )
                        .image_tint_follows_text_color(true),
                    )
                    .on_disabled_hover_text(if scanning {
                        "Wait for the current scan to finish."
                    } else {
                        "Choose a games folder with Browse first."
                    })
                    .clicked();
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!(
                            "../../assets/centralpanel/refresh/refresh-compat.svg"
                        ),
                        "Refresh Compatibility",
                    )
                    .image_tint_follows_text_color(true),
                );
                let _ = ui.add(
                    Button::image_and_text(
                        egui::include_image!(
                            "../../assets/centralpanel/refresh/refresh-compat.svg"
                        ),
                        "Check for title updates",
                    )
                    .image_tint_follows_text_color(true),
                );
            });
        SidepanelActions {
            search: search_changed.then_some(self.search.query.as_str()),
            refresh_list,
        }
    }
}
