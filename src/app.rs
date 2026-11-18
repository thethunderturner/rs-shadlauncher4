use crate::config::emulator::Config;
use crate::gui::Gui;
use eframe::egui;

pub mod state;

#[derive(Default)]
pub struct App {
    gui: Gui,
    config: Config,
    state: state::AppState,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut app = Self {
            config: Config::load().unwrap_or_else(|error| {
                eprintln!("Failed to load configuration: {error}");
                Config::default()
            }),
            ..Self::default()
        };
        app.state.load(&cc.egui_ctx, false);
        app
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.state.poll(ui.ctx()) {
            self.gui.update_search(&self.state.titles, self.state.load_ms);
        }
        match self.gui.show(ui, &self.state, &mut self.config) {
            Some(crate::gui::toolbar::Action::RefreshTitles) => self.state.load(ui.ctx(), true),
            Some(crate::gui::toolbar::Action::RefreshCompatibility) => {
                self.state.refresh_compatibility(ui.ctx())
            }
            None => {}
        }
    }
}
