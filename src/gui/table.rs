use crate::config::emulator::Config;
use crate::config::storage::user_directory;
use crate::gui::windows::settings::SettingsWindow;
use crate::library::attrac9::Player;
use crate::library::{cache::Db, compatibility, scan::Title, sorting::Sorting};
use eframe::egui;
use eframe::egui::Ui;
use egui_extras::{Column, TableBuilder};
use std::collections::HashSet;

#[derive(Default)]
pub struct Table {
    pub load_ms: f64,
    selected: Option<usize>, // row index
    background: Option<egui::TextureHandle>,
    sorting: Sorting,
    sound: Player,
    custom_config: Option<Result<Option<Config>, String>>,
    custom_settings: Option<CustomSettings>,
    custom_config_serials: HashSet<String>,
}

struct CustomSettings {
    serial: String,
    window: SettingsWindow,
    config: Config,
}

impl Table {
    pub fn refresh_custom_configs(&mut self) {
        let result =
            user_directory().and_then(|directory| std::fs::read_dir(directory.join("custom_configs")));
        match result {
            Ok(entries) => {
                self.custom_config_serials = entries
                    .filter_map(Result::ok)
                    .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().is_some_and(|extension| extension == "json"))
                    .filter_map(|path| path.file_stem()?.to_str().map(str::to_owned))
                    .collect();
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.custom_config_serials.clear();
            }
            Err(error) => eprintln!("Could not check custom configs: {error}"),
        }
    }

    pub fn sort_indices(&self, titles: &[Title], indices: &mut [usize]) {
        self.sorting.sort(titles, indices);
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        titles: &[Title],
        loading: bool,
        error: Option<&str>,
        config: &Config,
        indices: &mut [usize], // indices after sorting, searching etc
    ) {
        if loading || !config.frontend.play_background_music {
            if self.selected.is_some() {
                self.sound.stop();
            }
        }
        self.sound.set_volume(config.frontend.background_music_volume);
        let mut play_title = None;
        let show_compatibility = config.frontend.show_compatibility;
        if loading {
            self.selected = None;
            self.background = None;
        }
        let mut clicked_title = None;
        let mut clicked_column = None;
        let shown = if loading
            || !config
                .general
                .install_dirs
                .iter()
                .any(|directory| directory.enabled)
        {
            0
        } else {
            indices.len()
        };
        egui::Panel::bottom("library_table_footer")
            .resizable(false)
            .show(ui, |ui| {
                ui.label(format!("{shown} Titles Shown ({:.2} ms)", self.load_ms))
                    .on_hover_text(
                        "Time spent loading or updating the shown titles, including filtering and sorting.",
                    );
            });
        egui::CentralPanel::default().show(ui, |ui| {
            if loading {
                ui.label("Loading game library");
                return;
            }
            if let Some(error) = error {
                ui.colored_label(ui.visuals().error_fg_color, format!("Could not load library: {error}"));
                if titles.is_empty() {
                    return;
                }
            }
            if !config
                .general
                .install_dirs
                .iter()
                .any(|directory| directory.enabled)
            {
                ui.label("Add or enable a game folder in Settings -> Paths.");
                return;
            }
            if titles.is_empty() {
                ui.label("No games found. Use Refresh Titles to scan the selected folders.");
                return;
            }
            if indices.is_empty() {
                ui.label("No games match your search or filter. Clear the search or select All.");
                return;
            }
            if config.frontend.show_background_image {
                if let Some(background) = &self.background {
                    let rect = ui.max_rect();
                    let size = background.size_vec2();
                    let scale = (rect.width() / size.x).max(rect.height() / size.y);
                    let uv = egui::Rect::from_center_size(egui::pos2(0.5, 0.5), rect.size() / (size * scale));
                    let opacity = config.frontend.background_image_opacity.min(100) as f32 / 100.0;
                    ui.painter().image(
                        background.id(),
                        rect,
                        uv,
                        egui::Color32::WHITE.gamma_multiply(opacity),
                    );
                }
            }
            // Row fills use 60% opacity
            let visuals = ui.visuals_mut();
            let [r, g, b, _] = visuals.selection.bg_fill.to_srgba_unmultiplied();
            visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(r, g, b, 153);
            let [r, g, b, _] = visuals.widgets.hovered.bg_fill.to_srgba_unmultiplied();
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(r, g, b, 153);

            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .sense(egui::Sense::click())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(60.0)) // icon
                .column(Column::initial(210.0).at_least(120.0).at_most(300.0)); // name
            if show_compatibility {
                table = table
                    .column(Column::exact(100.0)) // compatibility status
                    .column(Column::exact(100.0)); // milestone
            }
            table
                .column(Column::exact(90.0)) // serial
                .column(Column::exact(75.0)) // region
                .column(Column::exact(75.0)) // fw
                .column(Column::exact(75.0)) // version
                .column(Column::exact(75.0)) // features (eg VR, Neo)
                .column(Column::remainder().at_least(320.0)) // path
                .header(28.0, |mut header| {
                    for (column, label, sortable) in [
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
                        if !show_compatibility && matches!(column, "status" | "milestone") {
                            continue;
                        }
                        header.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.add_space(8.0);
                                if sortable {
                                    if ui.add(self.sorting.header(column, label)).clicked() {
                                        clicked_column = Some(column);
                                    }
                                } else {
                                    ui.add(egui::Label::new(label).truncate().selectable(false));
                                }
                            });
                        });
                    }
                })
                .body(|body| {
                    body.rows(60.0, indices.len(), |mut row| {
                        let index = indices[row.index()];
                        let title = &titles[index];
                        row.set_selected(self.selected == Some(index));

                        row.col(|ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    if let Some(icon) = &title.icon {
                                        ui.add(
                                            egui::Image::from_bytes(
                                                format!("bytes://title-icon/{}", title.serial),
                                                std::sync::Arc::clone(icon),
                                            )
                                            .corner_radius(5.0)
                                            .fit_to_exact_size(egui::vec2(55.0, 55.0)),
                                        );
                                    } else {
                                        ui.add(egui::Label::new("?").selectable(false))
                                            .on_hover_text("Icon unavailable");
                                    }
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.add_space(8.0);
                                if self.custom_config_serials.contains(&title.serial) {
                                    ui.add(
                                        egui::Image::new(egui::include_image!("../assets/settings.svg"))
                                            .fit_to_exact_size(egui::vec2(24.0, 24.0))
                                            .alt_text("Game-specific settings"),
                                    )
                                    .on_hover_text("Game-specific settings");
                                }
                                ui.add(
                                    egui::Label::new(&title.name)
                                        .truncate()
                                        .halign(egui::Align::LEFT)
                                        .selectable(false),
                                );
                            });
                        });
                        if show_compatibility {
                            row.col(|ui| {
                                let status = title
                                    .compatibility
                                    .as_deref()
                                    .filter(|status| !status.is_empty())
                                    .unwrap_or("Unknown");
                                ui.add_space(8.0);
                                let (rect, _) =
                                    ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                                ui.painter().circle_filled(
                                    rect.center(),
                                    5.0,
                                    compatibility::status_color(status),
                                );
                                ui.add(egui::Label::new(status).truncate().selectable(false));
                            });
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(title.milestone.as_deref().unwrap_or("Unknown"))
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                });
                            });
                        }
                        row.col(|ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(&title.serial)
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    show_row_region(ui, &title.publisher_id);
                                },
                            );
                        }); // No region in the cache yet.
                        row.col(|ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(&format!(
                                            "{:02X}.{:02X}",
                                            (title.fw >> 24) & 0xff,
                                            (title.fw >> 16) & 0xff
                                        ))
                                        .truncate()
                                        .halign(egui::Align::LEFT)
                                        .selectable(false),
                                    );
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(&title.version)
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;

                                if title.supports_neo {
                                    ui.add(
                                        egui::Image::new(egui::include_image!(
                                            "../assets/table/features/pro.svg"
                                        ))
                                        .fit_to_exact_size(egui::vec2(32.0, 32.0))
                                        .tint(ui.visuals().text_color()),
                                    )
                                    .on_hover_text("Features for PS4 Pro");
                                }

                                if title.supports_vr {
                                    ui.add(
                                        egui::Image::new(egui::include_image!(
                                            "../assets/table/features/vr.svg"
                                        ))
                                        .fit_to_exact_size(egui::vec2(32.0, 32.0))
                                        .tint(ui.visuals().text_color()),
                                    )
                                    .on_hover_text("VR Support/Required");
                                }
                            });
                        });
                        row.col(|ui| {
                            if let Some(path) = title.app_path.as_ref().or(title.patch_path.as_ref()) {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.add_space(8.0);
                                    ui.add(
                                        egui::Label::new(path.to_string_lossy())
                                            .truncate()
                                            .halign(egui::Align::LEFT)
                                            .selectable(false),
                                    );
                                });
                            }
                        });
                        let response = row.response();
                        if !loading && (response.clicked() || response.secondary_clicked()) {
                            if response.clicked() || self.selected != Some(index) {
                                play_title = Some(title);
                            }
                            if self.selected != Some(index) {
                                self.selected = Some(index);
                                clicked_title = Some(title);
                            }
                            if response.secondary_clicked() {
                                self.custom_config = Some(match Config::load_custom(&title.serial) {
                                    Ok(config) => Ok(Some(config)),
                                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
                                    Err(error) => Err(error.to_string()),
                                });
                            }
                            response.ctx.request_repaint();
                        }
                        if !loading {
                            egui::Popup::context_menu(&response)
                                .id(egui::Id::unique((
                                    "game_row_menu",
                                    &title.serial,
                                    &title.app_path,
                                    &title.patch_path,
                                )))
                                .show(|ui| {
                                    if let Some(edit_config) = show_row_menu(
                                        ui,
                                        title,
                                        config,
                                        &mut self.custom_config,
                                        &mut self.custom_config_serials,
                                    ) {
                                        let mut window = SettingsWindow::default();
                                        window.begin_custom_edit(&title.serial, &edit_config);
                                        self.custom_settings = Some(CustomSettings {
                                            serial: title.serial.clone(),
                                            window,
                                            config: edit_config,
                                        });
                                    }
                                });
                        }
                    });
                });
        });
        if let Some(settings) = &mut self.custom_settings {
            let mut open = true;
            if settings.window.show(ui, &mut open, &mut settings.config, true) {
                self.custom_config_serials.insert(settings.serial.clone());
                ui.ctx().request_repaint();
            }
            if !open {
                self.custom_settings = None;
            }
        }
        if let Some(column) = clicked_column {
            let started = std::time::Instant::now();
            self.sorting.select_column(column);
            self.sort_indices(titles, indices);
            self.load_ms = started.elapsed().as_secs_f64() * 1000.0;
            ui.ctx().request_repaint();
        }
        if let Some(title) = play_title {
            if config.frontend.play_background_music {
                self.sound.play(title, config.frontend.background_music_volume);
            }
        }
        if let Some(title) = clicked_title {
            self.background = match load_background(ui.ctx(), &title.serial) {
                Ok(background) => background,
                Err(error) => {
                    eprintln!("Failed to load background for {}: {error}", title.serial);
                    None
                }
            };
            ui.ctx().request_repaint();
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

fn show_row_region(ui: &mut Ui, content_id: &str) {
    let (region, flag) = match content_id.get(..2) {
        Some("UP" | "UB" | "UT" | "US") => ("USA", egui::include_image!("../assets/table/regions/us.svg")),
        Some("EP" | "EB") => ("Europe", egui::include_image!("../assets/table/regions/eu.svg")),
        Some("JP" | "JB") => ("Japan", egui::include_image!("../assets/table/regions/jp.svg")),
        Some("HP" | "HB") => ("Asia", egui::include_image!("../assets/table/regions/cn.svg")),
        Some("IP") => ("World", egui::include_image!("../assets/table/regions/wr.svg")),
        _ => ("Unknown", egui::include_image!("../assets/table/regions/xx.svg")),
    };

    ui.add(
        egui::Image::new(flag)
            .fit_to_exact_size(egui::vec2(32.0, 32.0))
            .alt_text(region),
    )
    .on_hover_text(region);
}

fn show_row_menu(
    ui: &mut Ui,
    title: &Title,
    global_config: &Config,
    custom_config: &mut Option<Result<Option<Config>, String>>,
    custom_config_serials: &mut HashSet<String>,
) -> Option<Config> {
    let mut edit_config = None;
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
    ui.menu_button("Game Specific Settings", |ui| match custom_config.as_ref() {
        Some(Ok(Some(config))) => {
            if ui.button("Configure Game-Specific settings").clicked() {
                edit_config = Some(config.clone());
                ui.close();
            }
            if ui.button("Delete Game-Specific settings").clicked() {
                *custom_config = Some(match Config::delete_custom(&title.serial) {
                    Ok(()) => {
                        custom_config_serials.remove(&title.serial);
                        ui.ctx().request_repaint();
                        Ok(None)
                    }
                    Err(error) => Err(format!("Could not delete config: {error}")),
                });
            }
        }
        Some(Ok(None)) => {
            if ui
                .button("Create Game-Specific config from global config")
                .clicked()
            {
                edit_config = Some(global_config.clone());
                ui.close();
            }
        }
        Some(Err(error)) => {
            ui.colored_label(ui.visuals().error_fg_color, format!("Could not load custom config: {error}"));
        }
        None => {}
    });
    let _ = ui.button("Add to favorites");
    ui.menu_button("Add to folder", |ui| {
        let _ = ui.button("TODO");
    });
    let _ = ui.button("Cheats/Patches");
    let _ = ui.button("Trophy Viewer");
    ui.add_enabled(false, egui::Button::new("SFO Viewer"))
        .on_disabled_hover_text("SFO Viewer is not implemented yet.");
    ui.menu_button("Copy Info", |ui| {
        for (label, value) in [
            ("Name", &title.name),
            ("Serial", &title.serial),
            ("Publisher ID", &title.publisher_id),
        ] {
            if ui.button(label).clicked() {
                ui.ctx().copy_text(value.clone());
                ui.close();
            }
        }
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
    edit_config
}
