use crate::gui::centralpanel::actions::row::Row;
use crate::gui::centralpanel::sorting::{Order, Sorting};
use crate::scanning::Title;
use eframe::egui;

pub mod actions;
pub mod list;
pub mod sorting;

pub struct Centralpanel {
    titles: Vec<Title>,
    sorting: Sorting,
    loading: bool,
    scan_failed: bool,
    row: Row,
}

impl Default for Centralpanel {
    fn default() -> Self {
        Self {
            titles: Vec::new(),
            sorting: Sorting {
                column: String::from("icon"),
                order: Order::Descending,
            },
            loading: true,
            scan_failed: false,
            row: Row::default(),
        }
    }
}

impl Centralpanel {
    pub fn set_titles(&mut self, titles: Vec<Title>) {
        self.titles = titles;
        self.loading = false;
        self.scan_failed = false;
    }

    pub fn set_scan_failed(&mut self) {
        self.loading = false;
        self.scan_failed = true;
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
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
            } else {
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
