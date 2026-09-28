mod menubar;

use eframe::egui;

#[derive(Default)]
pub struct LauncherApp {
    show_about: bool,
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        menubar::show(ui, &mut self.show_about);

        egui::CentralPanel::default().show(ui, |_ui| {});

        if self.show_about {
            egui::Window::new("About shadLauncher")
                .open(&mut self.show_about)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("shadLauncher");
                    ui.label("A launcher for shadPS4.");
                });
        }
    }
}
