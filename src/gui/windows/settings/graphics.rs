use crate::config::emulator::Config;
use crate::devices::gpu::GpuDevice;
use eframe::egui;

pub struct GraphicsWindow {
    graphics_devices: Vec<GpuDevice>,
}

impl GraphicsWindow {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            graphics_devices: GpuDevice::scan()?,
        })
    }

    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new("Graphics Device").strong());
            columns[0].group(|ui| {
                ui.horizontal(|ui| {
                    let current = &mut config.vulkan.gpu_id;
                    let selected_gpu = self.graphics_devices.iter().find(|gpu| *current == gpu.id as i32);

                    egui::ComboBox::from_id_salt("graphics_device")
                        .selected_text(selected_gpu.map(|gpu| gpu.name.as_str()).unwrap_or("Auto"))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(current, -1, "Auto");
                            for gpu in &self.graphics_devices {
                                ui.selectable_value(current, gpu.id as i32, &gpu.name);
                            }
                        });

                    let tooltip = match selected_gpu {
                        Some(gpu) => format!(
                            "Name: {}\nVendor ID: {:#06X}\nDevice ID: {:#06X}\nType: {:?}",
                            gpu.name, gpu.vendor, gpu.device, gpu.device_type,
                        ),
                        None => "Automatically select the best available GPU".to_string(),
                    };

                    ui.add(
                        egui::Image::new(egui::include_image!("../../../assets/info.svg"))
                            .fit_to_exact_size(egui::vec2(16.0, 16.0))
                            .sense(egui::Sense::hover()),
                    )
                    .on_hover_text(tooltip);
                });
                ui.checkbox(&mut config.gpu.null_gpu, "Enable NULL GPU");
            });
            columns[0].label(egui::RichText::new("FSR Settings").strong());
            columns[0].group(|ui| {
                ui.checkbox(&mut config.general.show_splash, "Enable FSR");
                ui.checkbox(&mut config.general.discord_rpc_enabled, "Enable RCAS sharpening");
                ui.label(egui::RichText::new("RCAS attenuation").strong());
                ui.group(|ui| {
                    ui.add(
                        egui::Slider::new(&mut config.gpu.rcas_attenuation, 0..=3000)
                            .custom_formatter(|value, _| format!("{:.3}", value / 1000.0))
                            .show_value(true),
                    );
                });
            });
            columns[1].label(egui::RichText::new("Video").strong());
            columns[1].group(|ui| {
                ui.label(egui::RichText::new("Display Mode").strong());
                ui.group(|ui| {
                    let value = &mut config.gpu.full_screen_mode;
                    egui::ComboBox::from_id_salt("display_mode")
                        .selected_text(match value.as_str() {
                            "Windowed" => "Windowed",
                            "Fullscreen (Borderless)" => "Fullscreen (Borderless)",
                            "Fullscreen" => "Fullscreen",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, "Windowed".into(), "Windowed");
                            ui.selectable_value(
                                value,
                                "Fullscreen (Borderless)".into(),
                                "Fullscreen (Borderless)",
                            );
                            ui.selectable_value(value, "Fullscreen".into(), "Fullscreen");
                        });
                });
                ui.label(egui::RichText::new("Present Mode").strong());
                ui.group(|ui| {
                    let value = &mut config.gpu.present_mode;
                    egui::ComboBox::from_id_salt("present_mode")
                        .selected_text(match value.as_str() {
                            "Mailbox" => "Mailbox (Vsync)",
                            "Fifo" => "Fifo (Vsync)",
                            "Immediate" => "Immediate (No Vsync)",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, "Mailbox".into(), "Mailbox (Vsync)");
                            ui.selectable_value(value, "Fifo".into(), "Fifo (Vsync)");
                            ui.selectable_value(value, "Immediate".into(), "Immediate (No Vsync)");
                        });
                });
                ui.label(egui::RichText::new("Window Size").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut config.gpu.window_width)
                                .range(1..=16384)
                                .prefix("Width: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut config.gpu.window_height)
                                .range(1..=16384)
                                .prefix("Height: "),
                        );
                    });
                });
                ui.checkbox(&mut config.gpu.hdr_allowed, "Enable HDR");
            });
        });
    }
}
