use eframe::egui;
use egui_extras::{Column, TableBuilder};
use crate::gui::centralpanel::Centralpanel;

impl Centralpanel {
    pub fn show_list(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(1120.0);
            TableBuilder::new(ui)
                .sense(egui::Sense::click())
                .resizable(true)
                .cell_layout(egui::Layout::centered_and_justified(
                    egui::Direction::TopDown,
                ))
                .column(Column::exact(52.0))
                .column(Column::initial(210.0).at_least(120.0).at_most(300.0))
                .column(Column::exact(90.0))
                .column(Column::exact(75.0))
                .column(Column::exact(75.0))
                .column(Column::remainder().at_least(320.0))
                .header(24.0, |mut header| {
                    for label in ["Icon", "Name", "Serial", "Firmware", "Version", "Path"] {
                        header.col(|ui| {
                            if matches!(label, "Name" | "Path") {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(8.0);
                                        ui.strong(label);
                                    },
                                );
                            } else {
                                ui.strong(label);
                            }
                        });
                    }
                })
                .body(|body| {
                    body.rows(44.0, self.titles.len(), |mut row| {
                        let index = row.index();
                        let title = &self.titles[index];
                        let patch = title.patch.as_ref();

                        row.col(|ui| {
                            ui.add(egui::Label::new("?").selectable(false))
                                .on_hover_text("Icon unavailable");
                        });
                        row.col(|ui| {
                            let name = patch.map_or(title.app.name.as_str(), |p| &p.name);
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(name)
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.add(egui::Label::new(&title.serial).selectable(false));
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(patch.map_or(title.app.fw.as_str(), |p| &p.fw))
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(
                                    patch.map_or(title.app.version.as_str(), |p| &p.version),
                                )
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            let path = title.parent_dir.to_string_lossy();
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(path.as_ref())
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    )
                                        .on_hover_text(path.as_ref());
                                },
                            );
                        });
                    });
                });
        });
    }
}