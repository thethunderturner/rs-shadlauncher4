use crate::app::State;
use crate::gui::widgets::RowWidget;
use crate::library::attrac9::Player;
use crate::library::cache::Db;
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
            if !state.launcher.play_background_music {
                if self.sound.active {
                    self.sound.stop();
                }
            }
            self.show_background(ui, state);
            table_visuals(ui);
            let ctx = ui.ctx().clone();
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
                .body(|body| {
                    let titles = &state.loaded_titles;

                    body.rows(60.0, state.visible_indices.len(), |mut row| {
                        let index = state.visible_indices[row.index()];
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
                            self.background = match load_background(&ctx, &title.serial) {
                                Ok(background) => background,
                                Err(error) => {
                                    eprintln!("Failed to load background for {}: {error}", title.serial);
                                    None
                                }
                            };
                            if state.launcher.play_background_music {
                                self.sound.play(title, state.launcher.background_music_volume);
                            }
                            ctx.request_repaint();
                        }
                    });
                })
        });
    }

    fn show_background(&self, ui: &mut Ui, state: &State) {
        if state.launcher.show_background_image {
            if let Some(background) = &self.background {
                let rect = ui.max_rect();
                let size = background.size_vec2();
                let scale = (rect.width() / size.x).max(rect.height() / size.y);
                let uv = egui::Rect::from_center_size(egui::pos2(0.5, 0.5), rect.size() / (size * scale));
                let opacity = state.launcher.background_image_opacity.min(100) as f32 / 100.0;
                ui.painter()
                    .image(background.id(), rect, uv, egui::Color32::WHITE.gamma_multiply(opacity));
            }
        }
    }
}

fn load_background(
    ctx: &egui::Context,
    serial: &str,
) -> Result<Option<egui::TextureHandle>, Box<dyn std::error::Error>> {
    let db = Db::connect()?;
    let Some(bytes) = db.load_background(serial)? else {
        return Ok(None);
    };
    let image = image::load_from_memory(&bytes)?.to_rgba8();
    let image = egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image.as_raw(),
    );
    Ok(Some(ctx.load_texture("selected-title-background", image, egui::TextureOptions::LINEAR)))
}
