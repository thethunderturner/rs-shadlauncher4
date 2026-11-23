use crate::app::State;
use eframe::egui;

#[derive(Default)]
pub struct Footer;

impl Footer {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::Panel::bottom("library_table_footer")
            .resizable(false)
            .show(ui, |ui| {
                ui.label(format!(
                    "{} Titles Shown ({} ms)",
                    state.loaded_titles.loaded_titles.len(),
                    state.loaded_titles.load_ms
                ))
                .on_hover_text(
                    "Time spent loading or updating the shown titles, including filtering and sorting.",
                );
            });
    }
}
