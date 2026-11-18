use crate::config::emulator::Config;
use eframe::egui;
use eframe::egui::Button;

#[derive(Default)]
pub struct LogWindow;

impl LogWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Logger").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.log.enable, "Enable Logging");
                if ui.add(Button::new("Open Log Location")).clicked() {}
                ui.checkbox(&mut config.log.separate, "Separate Log Files");
                ui.checkbox(&mut config.log.sync, "Log Sync");
                ui.checkbox(&mut config.log.skip_duplicate, "Skip duplicate lines");
                ui.label(egui::RichText::new("Log Max Skip Duration").strong());
                if config.log.skip_duplicate {
                    ui.add(
                        egui::DragValue::new(&mut config.log.max_skip_duration)
                            .range(0..=u32::MAX)
                            .suffix("ms"),
                    );
                }
                ui.label(egui::RichText::new("Log Max Size Limit").strong());
                ui.add(
                    egui::DragValue::new(&mut config.log.size_limit)
                        .range(0..=u64::MAX)
                        .suffix("B"),
                );
                ui.checkbox(&mut config.log.append, "Log Append");
            });
            columns[1].label(egui::RichText::new("Log Filter").strong());
            columns[1].label("Not Available Yet");
        });
    }
}
