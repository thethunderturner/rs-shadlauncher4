use crate::config::emulator::Config;
use crate::config::storage::user_directory;
use eframe::egui;

#[derive(Default)]
pub struct UserWindow;

impl UserWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Trophy").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.general.trophy_popup_disabled, "Disable Trophy Notifications");
                ui.horizontal(|ui| {
                    let value = &mut config.general.trophy_notification_side;
                    ui.label(egui::RichText::new("Trophy Notification Position").strong());
                    egui::ComboBox::from_id_salt("trophy_notification_side")
                        .selected_text(match value.as_str() {
                            "top" => "Top",
                            "bottom" => "Bottom",
                            "left" => "Left",
                            "right" => "Right",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, "top".into(), "Top");
                            ui.selectable_value(value, "bottom".into(), "Bottom");
                            ui.selectable_value(value, "left".into(), "Left");
                            ui.selectable_value(value, "right".into(), "Right");
                        });
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Notification Duration").strong());
                    ui.add(
                        egui::DragValue::new(&mut config.general.trophy_notification_duration).range(0..=99),
                    );
                });
                if ui.button("Open custom trophy folder").clicked() {
                    if let Ok(path) = user_directory() {
                        let folder = path.join("custom_trophy");
                        if let Err(err) = open::that(&folder) {
                            eprintln!("Failed to open trophy folder: {err}");
                        }
                    }
                }
            });
            columns[1].label(egui::RichText::new("Portable User Folder").strong());
            columns[1].group(|ui| {
                ui.add_enabled(false, egui::Button::new("Create portable launcher folder"))
                    .on_disabled_hover_text("Portable launcher folders are not supported yet.");
            });
        });
    }
}
