use eframe::egui;

pub struct Help {
    pub show_about: bool,
}

impl Help {
    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            let _ = ui.button("Check For Updates");
            if ui.button("About").clicked() {
                self.show_about = true;
                ui.close();
            }
        });
    }

    pub(super) fn show_window(&mut self, ctx: &egui::Context) {
        if self.show_about {
            egui::Window::new("About shadLauncher")
                .open(&mut self.show_about)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("shadLauncher");
                    ui.label("A launcher for shadPS4.");
                });
        }
    }
}
