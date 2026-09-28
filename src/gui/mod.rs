mod menubar;
pub mod sidepanel;

use eframe::egui;

#[derive(Default)]
pub struct LauncherApp {
    show_about: bool,
    show_controllers: bool,
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        menubar::show(ui, &mut self.show_about, &mut self.show_controllers);

        sidepanel::show(ui);

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

        menubar::controllers::show_window(ui.ctx(), &mut self.show_controllers);
    }
}
