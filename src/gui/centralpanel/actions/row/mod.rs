use crate::gui::centralpanel::actions::row::sfo::sfo::{SfoViewer, Tab};
use crate::scanning::Title;
use atrac9::audio;

pub mod atrac9;
pub mod sfo;

#[derive(Default)]
pub struct Row {
    sfo_viewer: SfoViewer,
    audio: audio::Audio,
}

impl Row {
    pub fn play_audio(&mut self, title: &Title) {
        self.audio.play(title);
    }
    pub fn stop_audio(&self) {
        self.audio.stop();
    }
    pub fn open_sfo(&mut self, title: &Title) {
        if self.sfo_viewer.is_saving() {
            return;
        }
        self.sfo_viewer.reset_edits();
        self.sfo_viewer.title = title.clone();
        self.sfo_viewer.sfo_title = format!("{} ({})", title.app.name, title.serial);
        self.sfo_viewer.tab = Tab::App;
        self.sfo_viewer.open = true;
    }

    pub fn show_windows(&mut self, ctx: &eframe::egui::Context) {
        self.sfo_viewer.show(ctx);
    }

    pub fn take_sfo_saves(
        &mut self,
    ) -> Vec<(
        std::path::PathBuf,
        crate::sfo::internal::data_table::SfoDataTable,
    )> {
        self.sfo_viewer.take_saves()
    }

    pub fn is_saving_sfo(&self) -> bool {
        self.sfo_viewer.is_saving()
    }
}
