use crate::gui::centralpanel::actions::row::sfo::{SfoViewer, Tab};
use crate::scanning::Title;

pub mod sfo;

#[derive(Default)]
pub struct Row {
    sfo_viewer: SfoViewer,
}

impl Row {
    pub fn open_sfo(&mut self, title: &Title) {
        self.sfo_viewer.title = title.clone();
        self.sfo_viewer.sfo_title = format!("{} ({})", title.app.name, title.serial);
        self.sfo_viewer.tab = Tab::App;
        self.sfo_viewer.open = true;
    }

    pub fn show_windows(&mut self, ctx: &eframe::egui::Context) {
        self.sfo_viewer.show(ctx);
    }
}
