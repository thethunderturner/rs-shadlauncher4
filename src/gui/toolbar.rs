use crate::app::State;
use crate::gui::menubar::settings::Settings;
use crate::gui::windows::version_manager::VersionManager;
use eframe::egui;
use eframe::egui::ImageSource;

#[derive(Default)]
pub struct Toolbar {
    settings: Settings,
    version_manager: VersionManager,
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = "Version Manager";
                    let font = egui::TextStyle::Button.resolve(ui.style());
                    let text_width = ui
                        .painter()
                        .layout_no_wrap(label.to_owned(), font, ui.visuals().text_color())
                        .size()
                        .x;

                    let width = text_width + ui.spacing().button_padding.x * 2.0;
                    let control_height = ui.spacing().interact_size.y;
                    let gap = 8.0;

                    ui.allocate_ui_with_layout(
                        egui::vec2(width, control_height * 2.0 + gap),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.spacing_mut().item_spacing.y = gap;

                            ui.add_enabled_ui(false, |ui| {
                                egui::ComboBox::from_id_salt("installed_versions")
                                    .width(width)
                                    .selected_text("None")
                                    .show_ui(ui, |ui| {
                                        ui.label("No versions installed");
                                    })
                                    .response
                                    .on_disabled_hover_text("No installed version found");
                            });

                            if ui
                                .add_sized([width, control_height], egui::Button::new(label))
                                .clicked()
                            {
                                self.version_manager.open = true;
                            }
                        },
                    );
                });
            });
        });
        self.version_manager.show(ui, state);
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
