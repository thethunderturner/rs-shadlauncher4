use crate::gui::centralpanel::actions::row::Row;
use crate::gui::centralpanel::sorting::{Order, Sorting};
use crate::scanning::Title;
use eframe::egui;
use std::path::PathBuf;
use std::time::Duration;

pub mod actions;
pub mod table_list;
pub mod sorting;

pub struct Centralpanel {
    titles: Vec<Title>,
    column_sorting: Sorting,
    loading: bool,
    search: String,
    visible_titles: Vec<usize>,
    selected_title: Option<PathBuf>,
    reset_scroll: bool,
    scan_failed: bool,
    scan_duration: Option<Duration>,
    row: Row,
}

impl Default for Centralpanel {
    fn default() -> Self {
        Self {
            titles: Vec::new(),
            search: String::new(),
            visible_titles: Vec::new(),
            selected_title: None,
            reset_scroll: false,
            column_sorting: Sorting::default(),
            loading: true,
            scan_failed: false,
            scan_duration: None,
            row: Row::default(),
        }
    }
}

impl Centralpanel {
    pub fn start_scan(&mut self, ctx: &egui::Context) {
        actions::assets::clear_cache(ctx, &self.titles);
        self.loading = true;
        self.scan_failed = false;
        self.scan_duration = None;
    }

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
        self.scan_failed = false;
        self.scan_duration = Some(duration);
    }

    pub fn set_scan_failed(&mut self) {
        self.loading = false;
        self.scan_failed = true;
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("titles_footer")
            .resizable(false)
            .show(ui, |ui| {
                let shown = if self.loading || self.scan_failed {
                    0
                } else {
                    self.visible_titles.len()
                };
                if let Some(duration) = self.scan_duration {
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
            } else if self.scan_failed {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    "Could not scan the games folder.",
                );
            } else if self.titles.is_empty() {
                ui.label("No games found in the games folder.");
            } else if self.visible_titles.is_empty() {
                ui.label("No games match your search.");
                ui.label("Try another title or CUSA, or clear the search.");
            } else {
                if let Some(title) = self
                    .visible_titles
                    .iter()
                    .map(|&index| &self.titles[index])
                    .find(|title| Some(&title.app.path) == self.selected_title.as_ref())
                {
                    actions::assets::show_background(ui, title);
                }
                self.show_list(ui);
            }
        });
        self.row.show_windows(ui.ctx());
        for (path, sfo) in self.row.take_sfo_saves() {
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
