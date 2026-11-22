use crate::config::emulator::Config;
use crate::config::launcher::Launcher;
use crate::gui::Gui;
use crate::library::cache::LoadedTitles;
use eframe::egui;

#[derive(Default)]
pub struct App {
    pub gui: Gui,
    pub state: State,
}

#[derive(Default)]
pub struct State {
    pub config: Config,
    pub launcher: Launcher, // launcher config
    pub loaded_titles: LoadedTitles,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let app = Self {
            gui: Default::default(),
            state: State {
                config: Config::load(),
                launcher: Launcher::load(),
                loaded_titles: LoadedTitles::load(),
            },
        };
        app
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.gui.show(ui, &mut self.state)
    }
}
