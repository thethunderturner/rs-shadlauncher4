use crate::app::state::AppState;
use crate::config::emulator::Config;
use crate::library::scan::Title;
use eframe::egui;

pub mod dialogs;
pub mod menubar;
pub mod settings;
pub mod sidebar;
pub mod table;
pub mod toolbar;
pub mod widgets;
pub mod windows;

#[derive(Default)]
pub struct Gui {
    menubar: menubar::MenuBar,
    toolbar: toolbar::Toolbar,
    sidebar: sidebar::Sidebar,
    table: table::Table,
}

impl Gui {
    pub fn update_search(&mut self, titles: &[Title], load_ms: f64) {
        self.table.refresh_custom_configs();
        let started = std::time::Instant::now();
        self.sidebar.indices = self.sidebar.search(titles);
        self.table.sort_indices(titles, &mut self.sidebar.indices);
        self.table.load_ms = load_ms + started.elapsed().as_secs_f64() * 1000.0;
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        state: &AppState,
        config: &mut Config,
    ) -> Option<toolbar::Action> {
        self.menubar.show(ui, config);
        if self.sidebar.show(ui) {
            let started = std::time::Instant::now();
            self.sidebar.indices = self.sidebar.search(&state.titles);
            self.table.sort_indices(&state.titles, &mut self.sidebar.indices);
            self.table.load_ms = started.elapsed().as_secs_f64() * 1000.0;
        }
        let refresh = self
            .toolbar
            .show(ui, state.is_busy(), state.compatibility_message.as_deref(), config);
        self.table.show(
            ui,
            &state.titles,
            state.is_loading(),
            state.error.as_deref(),
            config,
            &mut self.sidebar.indices,
        );
        if let Some(progress) = &state.progress {
            let progress = progress.lock().ok().map(|progress| progress.clone());
            if let Some(progress) = progress {
                windows::progress::show(ui.ctx(), &progress);
            }
        }
        refresh
    }
}
