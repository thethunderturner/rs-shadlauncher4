use eframe::egui;

#[derive(Default)]
pub struct AboutWindow {
    pub open: bool,
}

impl AboutWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if !self.open {
            return;
        }

        egui::Window::new("About rs-shadLauncher4")
            .id(egui::Id::unique("about_launcher"))
            .open(&mut self.open)
            .default_width(580.0)
            .min_width(380.0)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE.inner_margin(16.0).show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 24.0;
                    ui.horizontal_top(|ui| {
                        let icon_size = (ui.available_width() * 0.32).min(180.0);
                        ui.add(
                            egui::Image::new(egui::include_image!("../../assets/icon.png"))
                                .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                .corner_radius(icon_size * 0.25)
                                .alt_text("rs-shadLauncher4 launcher icon"),
                        );
                        ui.vertical(|ui| {
                            ui.set_width(ui.available_width());
                            ui.style_mut().text_styles.insert(
                                egui::TextStyle::Heading,
                                egui::FontId::proportional(28.0),
                            );
                            ui.heading("rs-shadLauncher4");
                            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                            ui.add_space(12.0);
                            ui.add(egui::Label::new(
                                "A Rust launcher for the shadPS4 PlayStation 4 emulator."
                            ).wrap());
                            ui.add_space(8.0);
                            ui.add(egui::Label::new(
                                "Browse your game library, check compatibility, and manage global and game-specific settings."
                            ).wrap());
                        });
                    });
                });
            });
    }
}
