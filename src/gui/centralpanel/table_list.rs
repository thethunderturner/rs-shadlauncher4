use crate::gui::centralpanel::Centralpanel;
use crate::gui::centralpanel::actions::assets;
use crate::gui::centralpanel::actions::row::Row;
use crate::scanning::Title;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

impl Centralpanel {
    pub fn set_search(&mut self, search: &str) {
        self.search = search.trim().to_lowercase();
        self.filter_titles();
    }

    pub fn filter_titles(&mut self) {
        self.visible_titles = self
            .titles
            .iter()
            .enumerate()
            .filter_map(|(index, title)| {
                let matches = self.search.is_empty()
                    || title.app.name.to_lowercase().contains(&self.search)
                    || title
                        .patch
                        .as_ref()
                        .is_some_and(|patch| patch.name.to_lowercase().contains(&self.search))
                    || title.serial.to_lowercase().contains(&self.search);
                matches.then_some(index)
            })
            .collect();
        self.reset_scroll = true;
    }

    pub fn show_list(&mut self, ui: &mut egui::Ui) {
        let mut clicked_column = None;
        let mut clicked_title = None;
        let reset_scroll = std::mem::take(&mut self.reset_scroll);
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(1120.0);
            let mut table = TableBuilder::new(ui);
            if reset_scroll {
                table = table.vertical_scroll_offset(0.0);
            }
            table
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
                    for (column, label) in [
                        ("icon", "Icon"),
                        ("name", "Name"),
                        ("serial", "Serial"),
                        ("firmware", "Firmware"),
                        ("version", "Version"),
                        ("path", "Path"),
                    ] {
                        header.col(|ui| {
                            let mut show_button = |ui: &mut egui::Ui| {
                                let response = ui.add(self.column_sorting.header(column, label));
                                if response.clicked() {
                                    clicked_column = Some(column);
                                }
                                if column == "icon" {
                                    response.on_hover_text("Sort by icon availability");
                                }
                            };
                            if matches!(column, "name" | "path") {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(8.0);
                                        show_button(ui);
                                    },
                                );
                            } else {
                                show_button(ui);
                            }
                        });
                    }
                })
                .body(|body| {
                    body.rows(44.0, self.visible_titles.len(), |mut row| {
                        let index = self.visible_titles[row.index()];
                        let title = &self.titles[index];
                        let patch = title.patch.as_ref();
                        row.set_selected(self.selected_title.as_ref() == Some(&title.app.path));

                        row.col(|ui| {
                            if let Some(icon) = assets::load_icon(title) {
                                ui.add(icon);
                            } else {
                                ui.add(egui::Label::new("?").selectable(false))
                                    .on_hover_text("Icon unavailable");
                            }
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
                        let response = row.response();
                        if response.clicked() || response.secondary_clicked() {
                            clicked_title = Some(title.app.path.clone());
                        }
                        egui::Popup::context_menu(&response)
                            .id(egui::Id::new(("game_row_menu", &title.app.path)))
                            .show(|ui| show_row_menu(ui, title, &mut self.row));
                    });
                });
        });
        if let Some(path) = clicked_title {
            if self.selected_title.as_ref() != Some(&path) {
                if let Some(previous) = self
                    .titles
                    .iter()
                    .find(|title| Some(&title.app.path) == self.selected_title.as_ref())
                {
                    assets::forget_background(ui.ctx(), previous);
                }
                self.selected_title = Some(path);
                ui.ctx().request_repaint();
            }
        }
        if let Some(column) = clicked_column {
            self.column_sorting.select_column(column);
            self.column_sorting.sort(&mut self.titles);
            self.filter_titles();
            ui.ctx().request_repaint();
        }
    }
}

fn show_row_menu(ui: &mut egui::Ui, title: &Title, actions: &mut Row) {
    ui.menu_button("Launch", |ui| {
        let _ = ui.button("Launch with game specific configs (default)");
        let _ = ui.button("Launch with global configs only");
        let _ = ui.button("Launch with default settings");
    });
    ui.menu_button("Open Folder", |ui| {
        let _ = ui.button("Game Folder");
        let _ = ui.button("Update folder");
        let _ = ui.button("Log Folder");
        ui.menu_button("Save Data folder", |ui| {
            let _ = ui.button("Unimplemented user 1");
            let _ = ui.button("Unimplemented user 2");
            let _ = ui.button("Unimplemented user 3");
            let _ = ui.button("Unimplemented user 4");
        });
    });
    ui.menu_button("Game Specific Settings", |ui| {
        let _ = ui.button("TODO");
    });
    let _ = ui.button("Add to favorites");
    ui.menu_button("Add to folder", |ui| {
        let _ = ui.button("TODO");
    });
    let _ = ui.button("Cheats/Patches");
    let _ = ui.button("Trophy Viewer");
    if ui
        .add_enabled(!actions.is_saving_sfo(), egui::Button::new("SFO Viewer"))
        .on_disabled_hover_text("Wait for the SFO save to finish.")
        .clicked()
    {
        actions.open_sfo(title);
        ui.close();
    }
    ui.menu_button("Copy Info", |ui| {
        let _ = ui.button("Name");
        let _ = ui.button("Serial");
        let _ = ui.button("Publisher ID");
    });
    ui.menu_button("Delete", |ui| {
        let _ = ui.button("TODO");
    });
    ui.menu_button("Compatibility", |ui| {
        let _ = ui.button("TODO");
    });
    ui.menu_button("ZAR compression", |ui| {
        let _ = ui.button("TODO");
    });
}
