use crate::app::State;
use crate::gui::windows::about::AboutWindow;
use crate::gui::windows::update::UpdateWindow;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct Help {
    about_window: AboutWindow,
    update_window: UpdateWindow,
    startup_checked: bool,
}

impl Help {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) {
        if !self.startup_checked {
            self.startup_checked = true;
            self.update_window.channel = state.launcher.update_channel;
            if state.launcher.check_for_updates {
                self.update_window.check_at_startup(ui.ctx());
            }
        }
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
                self.update_window.start(ui.ctx());
                ui.close();
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
        self.update_window.show(ui, &mut state.launcher);
    }
}
