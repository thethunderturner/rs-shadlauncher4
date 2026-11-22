use crate::config::emulator::Config;
use crate::config::launcher::Launcher;
use crate::gui::Gui;
use crate::library::cache::LoadedTitles;
use crate::themes::AppTheme;
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
    pub visible_indices: Vec<usize>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let titles = LoadedTitles::load();
        let visible_indices = (0..titles.loaded_titles.len()).collect();
        let app = Self {
            gui: Default::default(),
            state: State {
                config: Config::load(),
                launcher: Launcher::load(),
                loaded_titles: titles,
                visible_indices,
            },
        };
        let theme = AppTheme::from_id(app.state.launcher.theme).unwrap_or(AppTheme::Dark);
        apply_theme(&cc.egui_ctx, theme);
        app
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let previous_theme = self.state.launcher.theme;
        self.gui.show(ui, &mut self.state);

        if self.state.launcher.theme != previous_theme {
            let theme = AppTheme::from_id(self.state.launcher.theme).unwrap_or(AppTheme::Dark);
            apply_theme(ui.ctx(), theme);
            ui.ctx().request_repaint();
        }
    }
}

// We check if the theme changed because we don't want to re-apply at every frame
fn apply_theme(ctx: &egui::Context, theme: AppTheme) {
    let egui_theme = match theme {
        AppTheme::Light => egui::Theme::Light,
        AppTheme::Dark => egui::Theme::Dark,
    };
    ctx.set_visuals_of(egui_theme, theme.visuals());
    ctx.set_theme(egui_theme);
}
