use eframe::egui;
use crate::app::State;

#[derive(Default)]
pub struct VersionManager {
    pub open: bool
}

impl VersionManager {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) -> bool {
        if !self.open {
            return false;
        }

        egui::Window::new("Version Manager")
            .id(egui::Id::unique("version_manager"))
            .open(&mut self.open)
            .default_size(egui::vec2(600.0, 400.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label(egui::RichText::new("Installed Versions directory").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let mut path = state.launcher.version_path.to_string_lossy().into_owned();
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Browse").clicked() {
                                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                    state.config.general.addon_install_dir = folder;
                                }
                            }
                            ui.add(
                                egui::TextEdit::singleline(&mut path)
                                    .interactive(false)
                                    .desired_width(f32::INFINITY)
                                    .background_color(ui.visuals().extreme_bg_color),
                            );
                        });
                    });
                });
            });
        self.open
    }
}