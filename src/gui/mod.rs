pub mod menubar;
pub mod sidepanel;

use crate::gui::menubar::Menubar;
use crate::gui::sidepanel::Sidepanel;
use eframe::egui;

#[derive(Default)]
pub struct LauncherApp {
    pub menubar: Menubar,
    pub sidepanel: Sidepanel,
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menubar.show(ui);
        self.sidepanel.show(ui);

        egui::CentralPanel::default().show(ui, |_ui| {});

        self.menubar.show_windows(ui.ctx());
    }
}
