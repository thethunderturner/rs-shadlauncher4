use crate::config::emulator::Config;
use eframe::egui;

#[derive(Default)]
pub struct ExperimentalWindow;

impl ExperimentalWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Experimental Features").strong());
            columns[0].group(|ui| {
                let options = &["Disabled", "Enabled", "Hybrid"];
                let value = &mut config.gpu.readbacks_mode;
                egui::ComboBox::from_id_salt("readback_mode")
                    .selected_text(options.get(*value as usize).copied().unwrap_or("Unknown"))
                    .show_ui(ui, |ui| {
                        for (index, option) in options.iter().enumerate() {
                            ui.selectable_value(value, index as u32, *option);
                        }
                    });
                ui.checkbox(&mut config.gpu.readback_linear_images_enabled, "Enable Readback Linear Images");
                ui.checkbox(&mut config.gpu.direct_memory_access_enabled, "Enable Direct Memory Access");
                ui.checkbox(&mut config.general.dev_kit_mode, "Enable DevKit Console Mode");
                ui.checkbox(&mut config.general.neo_mode, "Enable PS4 Pro");
                ui.checkbox(&mut config.general.neo_mode, "Enable Shader Cache");
            });
            columns[1].group(|ui| {
                ui.label(egui::RichText::new("Additional DMem Allocation").strong());
                ui.group(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut config.general.extra_dmem_in_mbytes)
                            .range(0..=100000)
                            .suffix("MB"),
                    );
                });
                ui.label(egui::RichText::new("VBlank Frequency").strong());
                ui.group(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut config.gpu.vblank_frequency)
                            .range(60..=9999)
                            .suffix("Hz"),
                    );
                });
            });
        });
    }
}
