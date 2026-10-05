use crate::scanning::Title;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use crate::gui::centralpanel::sorting::{Order, Sorting};

pub mod list;
pub mod sorting;
pub mod toolbar;

pub struct Centralpanel {
    titles: Vec<Title>,
    sorting: Sorting,
    loading: bool,
    scan_failed: bool,
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
    }
}
