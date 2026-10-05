use std::path::{Path, PathBuf};
use crate::scanning::Title;
use eframe::egui;
use crate::sfo::sfo::read_sfo;

pub struct SfoViewer {
    pub open: bool,
    pub sfo_title: String,
    pub title: Title,
    pub tab: Tab,
}

#[derive(PartialEq, Eq)]
pub enum Tab {
    App,
    Patch,
}

impl Default for SfoViewer {
    fn default() -> Self {
        Self {
            open: false,
            sfo_title: String::new(),
            title: Title::default(),
            tab: Tab::App,
        }
    }
}

impl SfoViewer {
    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return;
        }
        egui::Window::new(&self.sfo_title)
            .id(egui::Id::new("sfo_viewer"))
            .open(&mut self.open)
            .default_size([800.0, 500.0])
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.tab, Tab::App, "App");
                    if self.title.patch.is_some() {
                        ui.selectable_value(&mut self.tab, Tab::Patch, "Patch");
                    }
                });
                ui.separator();
                if self.tab == Tab::App {
                    Self::show_sfo_table(self.title.app.path.join("sce_sys").join("param.sfo"))
                } else if self.tab == Tab::Patch {
                    if let Some(patch) = self.title.patch.as_ref() {
                        Self::show_sfo_table(patch.path.join("sce_sys").join("param.sfo"))
                    }
                }
                ui.allocate_space(ui.available_size());
            });
    }

    pub fn show_sfo_table(path_buf: PathBuf) {
        let sfo = read_sfo(Path::new(&path_buf)).unwrap().data_table;
    }
}
