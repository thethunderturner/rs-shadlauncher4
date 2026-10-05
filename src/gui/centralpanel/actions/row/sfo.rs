use crate::scanning::Title;
use crate::sfo::data_table::{SfoDataTable, SfoValue};
use eframe::egui;
use egui_extras::{Column, TableBuilder};

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
                    ui.push_id(&self.title.app.path, |ui| {
                        Self::show_sfo_table(ui, &self.title.app.sfo);
                    });
                } else if self.tab == Tab::Patch {
                    if let Some(patch) = self.title.patch.as_ref() {
                        ui.push_id(&patch.path, |ui| {
                            Self::show_sfo_table(ui, &patch.sfo);
                        });
                    }
                }
            });
    }

    pub fn show_sfo_table(ui: &mut egui::Ui, sfo: &SfoDataTable) {
        if sfo.params.is_empty() {
            ui.label("This SFO contains no fields.");
            return;
        }
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(540.0);
            TableBuilder::new(ui)
                .id_salt("sfo_table")
                .striped(true)
                .resizable(true)
                .auto_shrink([false, false])
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(270.0))
                .column(Column::exact(270.0))
                .header(24.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Key");
                    });
                    header.col(|ui| {
                        ui.strong("Value");
                    });
                })
                .body(|body| {
                    body.rows(24.0, sfo.params.len(), |mut row| {
                        let param = &sfo.params[row.index()];
                        row.col(|ui| {
                            ui.add(egui::Label::new(&param.key).truncate())
                                .on_hover_text(&param.key);
                        });
                        row.col(|ui| {
                            let value = match &param.data {
                                SfoValue::Utf8(value) => value.clone(),
                                SfoValue::Integer(value) => value.to_string(),
                                SfoValue::Raw(_) => format!("{:?}", param.data),
                            };
                            ui.add(egui::Label::new(&value).truncate())
                                .on_hover_text(&value);
                        });
                    });
                });
        });
    }
}
