pub mod actions;

use eframe::egui;

#[derive(Default)]
pub struct Toolbar {}

impl Toolbar {
    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Panel::top("versions").resizable(false).show(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(110.0, 76.0),
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui| {
                        ui.add_space(4.0);
                        let width = ui.available_width();
                        ui.spacing_mut().combo_width = width;
                        ui.add_enabled_ui(false, |ui| {
                            egui::ComboBox::from_id_salt("installed_versions")
                                .selected_text("None")
                                .show_ui(ui, |ui| {
                                    ui.label("No versions installed");
                                })
                                .response
                                .on_disabled_hover_text("No installed version found");
                        });
                        ui.add_space(4.0);
                        let _ = ui.button("Version Manager");
                        ui.add_space(4.0);
                    },
                );
            });
        });
    }
}
