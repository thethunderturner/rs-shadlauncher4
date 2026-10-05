use crate::scanning::Title;
use eframe::egui;

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
                ui.allocate_space(ui.available_size());
            });
    }
}
