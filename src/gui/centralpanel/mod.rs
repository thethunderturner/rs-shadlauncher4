use crate::gui::centralpanel::actions::Actions;
use crate::gui::centralpanel::actions::scan::Scan;
use crate::gui::centralpanel::sorting::Sorting;
use crate::scanning::Title;
use eframe::egui;
use std::path::PathBuf;
use std::time::Duration;

pub mod actions;
pub mod sorting;
pub mod table_list;

pub struct Centralpanel {
    loading: bool,
    titles: Vec<Title>,
    filtered_titles: Vec<usize>, // stores the indexes of games matching the search
    column_sorting: Sorting,
    selected_title: Option<PathBuf>,
    search: String,
    reset_scroll: bool,
    scan: Scan,
    actions: Actions,
}

impl Default for Centralpanel {
    fn default() -> Self {
        Self {
            loading: false,
            titles: Vec::new(),
            filtered_titles: Vec::new(),
            column_sorting: Sorting::default(),
            search: String::new(),
            selected_title: None,
            reset_scroll: false,
            scan: Scan::default(),
            actions: Actions::default(),
        }
    }
}

impl Centralpanel {
    pub fn set_titles(&mut self, titles: Vec<Title>, duration: Duration) {
        self.titles = titles;
        if !self
            .titles
            .iter()
            .any(|title| Some(&title.app.path) == self.selected_title.as_ref())
        {
            self.selected_title = None;
        }
        self.column_sorting.sort(&mut self.titles);
        self.filter_titles();
        self.loading = false;
        self.scan.scan_failed = false;
        self.scan.scan_duration = Some(duration);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("titles_footer")
            .resizable(false)
            .show(ui, |ui| {
                let shown = if self.loading || self.scan.scan_failed {
                    0
                } else {
                    self.filtered_titles.len()
                };
                if let Some(duration) = self.scan.scan_duration {
                    ui.label(format!(
                        "{shown} Titles Shown ({} ms)",
                        duration.as_millis()
                    ))
                    .on_hover_text("Time spent scanning the games folder.");
                } else {
                    ui.label(format!("{shown} Titles Shown"));
                }
            });
        egui::CentralPanel::default().show(ui, |ui| {
            if self.loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Scanning titles");
                });
            } else if self.scan.scan_failed {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    "Could not scan the games folder.",
                );
            } else if self.titles.is_empty() {
                if self.scan.scan_duration.is_none() {
                    ui.label("Choose a games folder with Browse above.");
                } else {
                    ui.label("No games found in the selected folder.");
                }
            } else if self.filtered_titles.is_empty() {
                ui.label("No games match your search.");
                ui.label("Try another title or CUSA, or clear the search.");
            } else {
                if let Some(title) = self
                    .filtered_titles
                    .iter()
                    .map(|&index| &self.titles[index])
                    .find(|title| Some(&title.app.path) == self.selected_title.as_ref())
                {
                    actions::assets::show_background(ui, title);
                }
                self.show_list(ui);
            }
        });
        self.actions.row.show_windows(ui.ctx());
        for (path, sfo) in self.actions.row.take_sfo_saves() {
            for title in &mut self.titles {
                if title.app.path == path {
                    title.app.sfo = sfo.clone();
                } else if let Some(patch) = title.patch.as_mut() {
                    if patch.path == path {
                        patch.sfo = sfo.clone();
                    }
                }
            }
        }
    }
}
