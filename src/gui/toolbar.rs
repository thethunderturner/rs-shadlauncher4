use crate::app::State;
use crate::gui::menubar::settings::Settings;
use eframe::egui;
use eframe::egui::ImageSource;

#[derive(Default)]
pub struct Toolbar {
    settings: Settings,
}

impl Toolbar {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) -> bool {
        state.is_busy = state.scan.is_running() || state.compatibility.is_running();
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        button(ui, "Play", egui::include_image!("../assets/toolbar/play.svg"));
                        button(ui, "Pause", egui::include_image!("../assets/toolbar/pause.svg"));
                        button(ui, "Stop", egui::include_image!("../assets/toolbar/stop.svg"));
                        button(ui, "Restart", egui::include_image!("../assets/toolbar/restart.svg"));
                    });
                });
                ui.add(egui::Separator::default().vertical().shrink(8.0));
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled_ui(!state.is_busy, |ui| {
                                button(
                                    ui,
                                    "Refresh Titles",
                                    egui::include_image!("../assets/toolbar/refresh-titles.svg"),
                                )
                            })
                            .inner
                            .clicked()
                        {
                            state.scan.start(ui.ctx(), &state.config);
                            state.is_busy = state.scan.is_running() || state.compatibility.is_running();
                        }
                        if ui
                            .add_enabled_ui(!state.is_busy, |ui| {
                                button(
                                    ui,
                                    "Refresh Compatibility",
                                    egui::include_image!("../assets/toolbar/refresh-compatibility.svg"),
                                )
                            })
                            .inner
                            .clicked()
                        {
                            state.compatibility.start(ui.ctx());
                            state.is_busy = state.scan.is_running() || state.compatibility.is_running();
                        }
                    });
                });
                ui.add(egui::Separator::default().vertical().shrink(8.0));
                ui.group(|ui| {
                    if button(ui, "Settings", egui::include_image!("../assets/settings.svg")).clicked() {
                        if !self.settings.window.open {
                            self.settings.window.begin_edit(state);
                        }
                        self.settings.window.open = true;
                    };
                });
            });
        });
        self.settings.window.show(ui, state, false);
        let scanned_titles = state.scan.show(ui);
        let updated_titles = state.compatibility.show(ui);
        let was_busy = state.is_busy;
        state.is_busy = state.scan.is_running() || state.compatibility.is_running();
        if state.is_busy != was_busy {
            ui.ctx().request_repaint();
        }
        if let Some(titles) = scanned_titles.or(updated_titles) {
            state.visible_indices = (0..titles.loaded_titles.len()).collect();
            state.loaded_titles = titles;
            true
        } else {
            false
        }
    }
}

fn button(ui: &mut egui::Ui, hover: &str, image: ImageSource) -> egui::response::Response {
    let size = egui::vec2(48.0, 48.0);
    ui.add(egui::Button::image(egui::Image::new(image).fit_to_exact_size(size)))
        .on_hover_text(hover)
}
