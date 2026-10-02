pub mod list;
pub mod toolbar;

use crate::scanning::Title;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, TryRecvError};

pub struct Centralpanel {
    titles: Vec<Title>,
    icons: Vec<Option<egui::TextureHandle>>,
    selected_serial: Option<String>,
    background: Option<egui::TextureHandle>,
    background_rx: Option<Receiver<Option<egui::ColorImage>>>,
    loading: bool,
    scan_failed: bool,
}

pub(super) struct ScannedTitle {
    title: Title,
    icon: Option<egui::ColorImage>,
}

pub(super) fn prepare_titles(titles: Vec<Title>) -> Vec<ScannedTitle> {
    titles.into_iter().map(prepare_title).collect()
}

fn prepare_title(title: Title) -> ScannedTitle {
    let icon = title.icon_path.as_deref().and_then(load_icon);
    ScannedTitle { title, icon }
}

fn load_icon(path: &Path) -> Option<egui::ColorImage> {
    let image = image::open(path).ok()?.thumbnail(40, 40).to_rgba8();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image.as_raw(),
    ))
}

impl Centralpanel {
    pub(super) fn new() -> Self {
        Self {
            titles: Vec::new(),
            icons: Vec::new(),
            selected_serial: None,
            background: None,
            background_rx: None,
            loading: true,
            scan_failed: false,
        }
    }

    pub(super) fn set_titles(&mut self, ctx: &egui::Context, scanned: Vec<ScannedTitle>) {
        self.titles.clear();
        self.icons.clear();
        self.selected_serial = None;
        self.background = None;
        self.background_rx = None;
        for ScannedTitle { title, icon } in scanned {
            self.icons.push(icon.map(|image| {
                ctx.load_texture(
                    format!("icon-{}", title.serial),
                    image,
                    egui::TextureOptions::LINEAR,
                )
            }));
            self.titles.push(title);
        }
        self.loading = false;
    }

    pub(super) fn set_scan_failed(&mut self) {
        self.loading = false;
        self.scan_failed = true;
    }

    fn select_title(&mut self, index: usize, ctx: &egui::Context) {
        let title = &self.titles[index];
        if self.selected_serial.as_deref() == Some(&title.serial) {
            return;
        }
        self.selected_serial = Some(title.serial.clone());
        self.background = None;
        self.background_rx = None;
        if let Some(path) = title.background_path.clone() {
            let (tx, rx) = mpsc::channel();
            self.background_rx = Some(rx);
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                let image = load_background(&path);
                let _ = tx.send(image);
                ctx.request_repaint();
            });
        }
    }

    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(rx) = &self.background_rx {
            match rx.try_recv() {
                Ok(Some(image)) => {
                    self.background = Some(ui.ctx().load_texture(
                        "selected-game-background",
                        image,
                        egui::TextureOptions::LINEAR,
                    ));
                    self.background_rx = None;
                }
                Ok(None) | Err(TryRecvError::Disconnected) => {
                    self.background_rx = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(background) = &self.background {
                paint_background(ui, background);
            }
            if self.loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Scanning games!");
                });
                return;
            }
            if self.scan_failed {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    "Could not scan the games folder.",
                );
                return;
            }
            if self.titles.is_empty() {
                ui.label("No games found in the games folder.");
                return;
            }

            let mut clicked_title = None;
            let left_aligned = egui::Layout::left_to_right(egui::Align::Center);
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.set_min_width(1120.0);
                TableBuilder::new(ui)
                    .id_salt("games_table")
                    .striped(self.background.is_none())
                    .sense(egui::Sense::click())
                    .resizable(true)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::TopDown,
                    ))
                    .column(Column::exact(52.0))
                    .column(Column::initial(210.0).at_least(120.0))
                    .column(Column::exact(90.0))
                    .column(Column::exact(75.0))
                    .column(Column::exact(75.0))
                    .column(Column::remainder().at_least(320.0))
                    .header(24.0, |mut header| {
                        for label in ["Icon", "Name", "Serial", "Firmware", "Version", "Path"] {
                            header.col(|ui| {
                                if matches!(label, "Name" | "Path") {
                                    ui.with_layout(left_aligned, |ui| {
                                        ui.add_space(8.0);
                                        ui.strong(label);
                                    });
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
                            row.set_selected(
                                self.selected_serial.as_deref() == Some(&title.serial),
                            );

                            row.col(|ui| {
                                if let Some(icon) = &self.icons[index] {
                                    ui.add(
                                        egui::Image::from_texture(icon)
                                            .fit_to_exact_size(egui::vec2(40.0, 40.0)),
                                    );
                                } else {
                                    ui.add(egui::Label::new("?").selectable(false))
                                        .on_hover_text("Icon unavailable");
                                }
                            });
                            row.col(|ui| {
                                let name = patch.map_or(title.app.name.as_str(), |p| &p.name);
                                ui.with_layout(left_aligned, |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(name)
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                });
                            });
                            row.col(|ui| {
                                ui.add(egui::Label::new(&title.serial).selectable(false));
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        patch.map_or(title.app.fw.as_str(), |p| &p.fw),
                                    )
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
                                ui.with_layout(left_aligned, |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(path.as_ref())
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    )
                                    .on_hover_text(path.as_ref());
                                });
                            });
                            if row.response().clicked() {
                                clicked_title = Some(index);
                            }
                        });
                    });
            });
            if let Some(index) = clicked_title {
                self.select_title(index, ui.ctx());
            }
        });
    }
}

fn load_background(path: &Path) -> Option<egui::ColorImage> {
    let image = image::open(path).ok()?.to_rgba8();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image.as_raw(),
    ))
}

fn paint_background(ui: &egui::Ui, background: &egui::TextureHandle) {
    let rect = ui.max_rect();
    let size = background.size_vec2();
    let image_ratio = size.x / size.y;
    let rect_ratio = rect.width() / rect.height();
    let uv = if image_ratio > rect_ratio {
        let margin = (1.0 - rect_ratio / image_ratio) / 2.0;
        egui::Rect::from_min_max(egui::pos2(margin, 0.0), egui::pos2(1.0 - margin, 1.0))
    } else {
        let margin = (1.0 - image_ratio / rect_ratio) / 2.0;
        egui::Rect::from_min_max(egui::pos2(0.0, margin), egui::pos2(1.0, 1.0 - margin))
    };
    ui.painter()
        .image(background.id(), rect, uv, egui::Color32::WHITE);
    let overlay = if ui.visuals().dark_mode {
        egui::Color32::from_black_alpha(220)
    } else {
        egui::Color32::from_white_alpha(220)
    };
    ui.painter().rect_filled(rect, 0.0, overlay);
}
