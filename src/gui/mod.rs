pub mod menubar;
pub mod sidepanel;
pub mod centralpanel;

use crate::gui::menubar::Menubar;
use crate::gui::sidepanel::Sidepanel;
use eframe::egui;
use crate::gui::centralpanel::Centralpanel;

#[derive(Default)]
pub struct LauncherApp {
    pub menubar: Menubar,
    pub sidepanel: Sidepanel,
    pub centralpanel: Centralpanel
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menubar.show(ui);
        self.sidepanel.show(ui);
        self.centralpanel.show(ui);

        self.menubar.show_windows(ui.ctx());
    }
}
