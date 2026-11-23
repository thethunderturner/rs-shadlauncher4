use crate::app::State;
use crate::gui::windows::settings::SettingsWindow;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Settings {
    pub window: SettingsWindow,
}

impl Settings {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.menu_button("Settings", |ui| {
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/general.svg"),
                        "General",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("General");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/window.svg"),
                        "Frontend",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Frontend");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/graphics.svg"),
                        "Graphics",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Graphics");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/user.svg"),
                        "User",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {}
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/input.svg"),
                        "Input",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Input");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/network.svg"),
                        "Network",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Network");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/paths.svg"),
                        "Paths",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Paths");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/log.svg"),
                        "Log",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Log");
                self.window.open = true;
            }
            if ui
                .add(
                    Button::image_and_text(
                        egui::include_image!("../../assets/menubar/settings/debug.svg"),
                        "Debug",
                    )
                    .image_tint_follows_text_color(true),
                )
                .clicked()
            {
                if !self.window.open {
                    self.window.begin_edit(state);
                }
                self.window.select_tab("Debug");
                self.window.open = true;
            }
        });
        self.window.show(ui, state, false);
    }
}
