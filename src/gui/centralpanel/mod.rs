use crate::scanning::Title;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub struct Centralpanel {
    titles: Vec<Title>,
    icons: Vec<Option<egui::TextureHandle>>,
    loading: bool,
    scan_failed: bool,
}

pub(super) struct ScannedTitle {
    title: Title,
    icon: Option<egui::ColorImage>,
}

pub(super) fn prepare_titles(titles: Vec<Title>) -> Vec<ScannedTitle> {
    titles
        .into_iter()
        .map(|title| {
            let icon = (|| {
                let image = title
                    .patch
                    .as_ref()
                    .and_then(|patch| image::open(patch.path.join("sce_sys/icon0.png")).ok())
                    .or_else(|| image::open(title.app.path.join("sce_sys/icon0.png")).ok())?
                    .thumbnail(40, 40)
                    .to_rgba8();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [image.width() as usize, image.height() as usize],
                    image.as_raw(),
                );
                Some(color_image)
            })();
            ScannedTitle { title, icon }
        })
        .collect()
}

impl Centralpanel {
    pub(super) fn new() -> Self {
        Self {
            titles: Vec::new(),
            icons: Vec::new(),
            loading: true,
            scan_failed: false,
        }
    }

    pub(super) fn set_titles(&mut self, ctx: &egui::Context, scanned: Vec<ScannedTitle>) {
        self.titles.clear();
        self.icons.clear();
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

    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            if self.loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Scanning games…");
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

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.set_min_width(1120.0);
                TableBuilder::new(ui)
                    .id_salt("games_table")
                    .striped(true)
                    .resizable(true)
                    .column(Column::exact(52.0))
                    .column(Column::initial(210.0).at_least(120.0))
                    .column(Column::initial(110.0).at_least(90.0))
                    .column(Column::initial(90.0).at_least(75.0))
                    .column(Column::initial(90.0).at_least(75.0))
                    .column(Column::initial(90.0).at_least(75.0))
                    .column(Column::remainder().at_least(320.0))
                    .header(24.0, |mut header| {
                        for label in [
                            "Icon", "Name", "Serial", "Firmware", "Size", "Version", "Path",
                        ] {
                            header.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(44.0, self.titles.len(), |mut row| {
                            let index = row.index();
                            let title = &self.titles[index];
                            let patch = title.patch.as_ref();

                            row.col(|ui| {
                                if let Some(icon) = &self.icons[index] {
                                    ui.add(
                                        egui::Image::from_texture(icon)
                                            .fit_to_exact_size(egui::vec2(40.0, 40.0)),
                                    );
                                } else {
                                    ui.label("?").on_hover_text("Icon unavailable");
                                }
                            });
                            row.col(|ui| {
                                let name = patch.map_or(title.app.name.as_str(), |p| &p.name);
                                ui.add(egui::Label::new(name).truncate())
                                    .on_hover_text(name);
                            });
                            row.col(|ui| {
                                ui.label(&title.serial);
                            });
                            row.col(|ui| {
                                ui.label(patch.map_or(title.app.fw.as_str(), |p| &p.fw));
                            });
                            row.col(|ui| {
                                let bytes =
                                    title.app.size.saturating_add(patch.map_or(0, |p| p.size));
                                ui.label(format_size(bytes));
                            });
                            row.col(|ui| {
                                ui.label(patch.map_or(title.app.version.as_str(), |p| &p.version));
                            });
                            row.col(|ui| {
                                let path = title.parent_dir.to_string_lossy();
                                ui.add(egui::Label::new(path.as_ref()).truncate())
                                    .on_hover_text(path.as_ref());
                            });
                        });
                    });
            });
        });
    }
}

fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let bytes = bytes as f64;
    let (value, unit) = if bytes < KB.powi(2) {
        (bytes / KB, "KB")
    } else if bytes < KB.powi(3) {
        (bytes / KB.powi(2), "MB")
    } else if bytes < KB.powi(4) {
        (bytes / KB.powi(3), "GB")
    } else {
        (bytes / KB.powi(4), "TB")
    };
    format!("{value:.2} {unit}")
}
