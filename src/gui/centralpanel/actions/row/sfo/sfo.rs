use super::sfo_save::SfoSave;
use crate::scanning::Title;
use crate::sfo::internal::data_table::{SfoDataTable, SfoValue};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;

pub struct SfoViewer {
    pub open: bool,
    pub sfo_title: String,
    pub title: Title,
    pub tab: Tab,
    app_edit: Option<SfoEdit>,
    patch_edit: Option<SfoEdit>,
    app_save: SfoSave,
    patch_save: SfoSave,
    saved: Vec<(PathBuf, SfoDataTable)>,
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
            app_edit: None,
            patch_edit: None,
            app_save: SfoSave::default(),
            patch_save: SfoSave::default(),
            saved: Vec::new(),
        }
    }
}

impl SfoViewer {
    pub(crate) fn reset_edits(&mut self) {
        self.app_edit = None;
        self.patch_edit = None;
        self.app_save = SfoSave::default();
        self.patch_save = SfoSave::default();
    }

    pub(crate) fn is_saving(&self) -> bool {
        self.app_save.is_saving() || self.patch_save.is_saving()
    }

    pub(crate) fn take_saves(&mut self) -> Vec<(PathBuf, SfoDataTable)> {
        std::mem::take(&mut self.saved)
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if let Some(table) = self.app_save.poll() {
            self.saved.push((self.title.app.path.clone(), table));
        }
        if let Some(table) = self.patch_save.poll() {
            if let Some(patch) = &self.title.patch {
                self.saved.push((patch.path.clone(), table));
            }
        }
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
                        self.app_save.show(
                            ui,
                            &self.title.app.path,
                            &self.title.app.sfo,
                            self.app_edit.is_some(),
                        );
                        ui.add_enabled_ui(!self.app_save.is_saving(), |ui| {
                            Self::show_sfo_table(ui, &mut self.title.app.sfo, &mut self.app_edit);
                        });
                    });
                } else if self.tab == Tab::Patch {
                    if let Some(patch) = self.title.patch.as_mut() {
                        ui.push_id(&patch.path, |ui| {
                            self.patch_save.show(
                                ui,
                                &patch.path,
                                &patch.sfo,
                                self.patch_edit.is_some(),
                            );
                            ui.add_enabled_ui(!self.patch_save.is_saving(), |ui| {
                                Self::show_sfo_table(ui, &mut patch.sfo, &mut self.patch_edit);
                            });
                        });
                    }
                }
            });
    }

    fn show_sfo_table(ui: &mut egui::Ui, sfo: &mut SfoDataTable, editing: &mut Option<SfoEdit>) {
        if sfo.params.is_empty() {
            ui.label("This SFO contains no fields.");
            return;
        }
        if let Some(edit) = editing.as_ref() {
            if let Some(error) = &edit.error {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    format!("{}: {error}", edit.key),
                );
            }
        }
        let editor_id = ui.id().with("sfo_value_editor");
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(680.0);
            TableBuilder::new(ui)
                .id_salt("sfo_table")
                .striped(true)
                .resizable(true)
                .auto_shrink([false, false])
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(270.0))
                .column(
                    Column::initial(270.0)
                        .at_least(160.0)
                        .resizable(true)
                        .clip(true),
                )
                .column(Column::exact(120.0))
                .header(24.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Key");
                    });
                    header.col(|ui| {
                        ui.strong("Value");
                    });
                    header.col(|ui| {
                        ui.strong("Actions");
                    });
                })
                .body(|body| {
                    body.rows(28.0, sfo.params.len(), |mut row| {
                        let param = &mut sfo.params[row.index()];
                        let is_editing = editing.as_ref().is_some_and(|edit| edit.key == param.key);
                        let mut apply = false;
                        let mut cancel = false;
                        row.col(|ui| {
                            ui.add(egui::Label::new(&param.key).truncate())
                                .on_hover_text(&param.key);
                        });
                        row.col(|ui| {
                            if let Some(edit) =
                                editing.as_mut().filter(|edit| edit.key == param.key)
                            {
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut edit.draft)
                                        .id(editor_id.with(&param.key))
                                        .desired_width(f32::INFINITY),
                                );
                                let response =
                                    if matches!(param.key.as_str(), "ATTRIBUTE" | "ATTRIBUTE2") {
                                        response.on_hover_text(ATTRIBUTE_BYTES_HELP)
                                    } else {
                                        response
                                    };
                                if edit.focus {
                                    response.request_focus();
                                    edit.focus = false;
                                }
                                if response.changed() {
                                    edit.error = None;
                                }
                                if response.has_focus() || response.lost_focus() {
                                    apply = ui.input_mut(|input| {
                                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                                    });
                                    cancel = ui.input_mut(|input| {
                                        input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
                                    });
                                }
                            } else {
                                let value = value_text(&param.key, &param.data);
                                let response = ui.add(egui::Label::new(&value).truncate());
                                if matches!(param.key.as_str(), "ATTRIBUTE" | "ATTRIBUTE2") {
                                    response.on_hover_text(ATTRIBUTE_BYTES_HELP);
                                } else {
                                    response.on_hover_text(&value);
                                }
                            }
                        });
                        row.col(|ui| {
                            if is_editing {
                                apply |= ui.button("Apply").clicked();
                                cancel |= ui.button("Cancel").clicked();
                            } else if ui
                                .add_enabled(editing.is_none(), egui::Button::new("Edit"))
                                .on_disabled_hover_text("Apply or cancel the current edit first.")
                                .clicked()
                            {
                                *editing = Some(SfoEdit {
                                    key: param.key.clone(),
                                    draft: value_text(&param.key, &param.data),
                                    error: None,
                                    focus: true,
                                });
                            }
                        });
                        if cancel {
                            *editing = None;
                        } else if apply {
                            if let Some(edit) = editing.as_mut() {
                                match parse_value(&param.key, &param.data, &edit.draft) {
                                    Ok(value) => {
                                        param.data = value;
                                        *editing = None;
                                    }
                                    Err(error) => {
                                        edit.error = Some(error);
                                        edit.focus = true;
                                    }
                                }
                            }
                        }
                    });
                });
        });
    }
}

// Allows you to edit values in param.sfo
struct SfoEdit {
    key: String,
    draft: String,
    error: Option<String>,
    focus: bool,
}

const ATTRIBUTE_BYTES_HELP: &str = "Enter four hexadecimal bytes in file order (little endian)";

fn value_text(key: &str, value: &SfoValue) -> String {
    match value {
        SfoValue::Utf8(value) => value.clone(),
        SfoValue::Integer(value) if matches!(key, "ATTRIBUTE" | "ATTRIBUTE2") => {
            let [a, b, c, d] = value.to_le_bytes();
            format!("{a:02X} {b:02X} {c:02X} {d:02X}")
        }
        SfoValue::Integer(value) => value.to_string(),
        SfoValue::Raw(_) => format!("{value:?}"),
    }
}

fn parse_value(key: &str, original: &SfoValue, text: &str) -> Result<SfoValue, String> {
    match original {
        SfoValue::Utf8(_) => {
            if text.contains('\0') {
                return Err("Text cannot contain a null character.".into());
            }
            Ok(SfoValue::Utf8(text.to_owned()))
        }
        SfoValue::Integer(_) if matches!(key, "ATTRIBUTE" | "ATTRIBUTE2") => {
            let mut parts = text.split_ascii_whitespace();
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                let part = parts.next().ok_or(ATTRIBUTE_BYTES_HELP)?;
                if part.len() != 2 || !part.bytes().all(|ch| ch.is_ascii_hexdigit()) {
                    return Err(ATTRIBUTE_BYTES_HELP.into());
                }
                *byte = u8::from_str_radix(part, 16).map_err(|_| ATTRIBUTE_BYTES_HELP)?;
            }
            if parts.next().is_some() {
                return Err(ATTRIBUTE_BYTES_HELP.into());
            }
            Ok(SfoValue::Integer(u32::from_le_bytes(bytes)))
        }
        SfoValue::Integer(_) => {
            let text = text.trim();
            let value =
                if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
                    u32::from_str_radix(hex, 16)
                } else {
                    text.parse::<u32>()
                };
            value.map(SfoValue::Integer).map_err(|_| {
                "Enter a whole number from 0 to 4294967295 (decimal or 0x hex).".into()
            })
        }
        SfoValue::Raw(_) => {
            let text = text.trim();
            let text = text
                .strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))
                .unwrap_or(text);
            let hex: String = text
                .chars()
                .filter(|ch| !ch.is_ascii_whitespace())
                .collect();
            if hex.len() % 2 != 0 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err("Enter pairs of hex digits, for example 00 AB FF.".into());
            }
            let mut bytes = Vec::with_capacity(hex.len() / 2);
            for index in (0..hex.len()).step_by(2) {
                let byte = u8::from_str_radix(&hex[index..index + 2], 16)
                    .map_err(|_| "Enter valid hex bytes.".to_string())?;
                bytes.push(byte);
            }
            Ok(SfoValue::Raw(bytes))
        }
    }
}
