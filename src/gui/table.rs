use crate::app::State;
use crate::gui::widgets::RowWidget;
use crate::library::attrac9::Player;
use crate::library::sorting::Sorting;
use crate::themes::table_visuals;
use eframe::egui;
use eframe::egui::Ui;
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct Table {
    selected: Option<usize>, // selected title index
    background: Option<egui::TextureHandle>,
    sorting: Sorting,
    sound: Player,
}

impl Table {
    pub fn show(&mut self, ui: &mut Ui, state: &mut State) {
        egui::CentralPanel::default().show(ui, |ui| {
            table_visuals(ui);
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .sense(egui::Sense::click())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(60.0)) // icon
                .column(Column::initial(210.0).at_least(120.0).at_most(300.0)) // name
                .column(Column::exact(100.0)) // compatibility status
                .column(Column::exact(100.0)) // milestone
                .column(Column::exact(90.0)) // serial
                .column(Column::exact(75.0)) // region
                .column(Column::exact(75.0)) // fw
                .column(Column::exact(75.0)) // version
                .column(Column::exact(75.0)) // features (eg VR, Neo)
                .column(Column::remainder().at_least(320.0)) // path
                .header(28.0, |mut header| {
                    for (_column, label, _sortable) in [
                        ("icon", "Icon", false),
                        ("name", "Name", true),
                        ("status", "Status", true),
                        ("milestone", "Milestone", true),
                        ("serial", "Serial", true),
                        ("region", "Region", true),
                        ("firmware", "Firmware", true),
                        ("version", "Version", true),
                        ("features", "Features", false),
                        ("path", "Path", true),
                    ] {
                        header.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.add_space(8.0);
                                ui.add(egui::Label::new(label).truncate().selectable(false));
                            });
                        });
                    }
                })
                .body(|mut body| {
                    let titles = &state.loaded_titles;

                    body.rows(60.0, titles.visible_indices.len(), |mut row| {
                        let index = titles.visible_indices[row.index()];
                        let loaded = &titles.loaded_titles[index];
                        let title = &loaded.title;

                        row.set_selected(self.selected == Some(index));

                        row.col(|ui| {
                            if let Some(icon) = &loaded.icon {
                                ui.add(
                                    egui::Image::from_bytes(
                                        format!("bytes://{}/icon.png", title.serial),
                                        icon.clone(),
                                    )
                                    .fit_to_exact_size(egui::vec2(48.0, 48.0)),
                                );
                            }
                        });
                        row.col(|ui| {
                            RowWidget::label(ui, &title.name);
                        });
                        row.col(|ui| {
                            RowWidget::label(ui, loaded.compatibility.as_deref().unwrap_or("Unknown"));
                        });
                        row.col(|ui| {
                            RowWidget::label(ui, title.milestone.as_deref().unwrap_or("Unknown"));
                        });
                        row.col(|ui| {
                            RowWidget::label(ui, &title.serial);
                        });
                        row.col(|ui| {
                            RowWidget::region(ui, &title.publisher_id);
                        });
                        row.col(|ui| {
                            let text =
                                format!("{:02X}.{:02X}", (title.fw >> 24) & 0xff, (title.fw >> 16) & 0xff);
                            RowWidget::label(ui, &text);
                        });
                        row.col(|ui| {
                            RowWidget::label(ui, &title.version);
                        });
                        row.col(|ui| {
                            RowWidget::features(ui, &title);
                        });
                        row.col(|ui| {
                            if let Some(path) = title.app_path.as_ref().or(title.patch_path.as_ref()) {
                                ui.add(egui::Label::new(path.to_string_lossy()).truncate());
                            }
                        });

                        if row.response().clicked() {
                            self.selected = Some(index);
                        }
                    });
                })
        });
    }
}
