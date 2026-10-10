use eframe::egui;
use serde_json::{Map, Value};

pub fn two_columns(
    ui: &mut egui::Ui,
    config: &mut Value,
    left: impl FnOnce(&mut egui::Ui, &mut Value),
    right: impl FnOnce(&mut egui::Ui, &mut Value),
) {
    if ui.available_width() < 650.0 {
        left(ui, config);
        ui.add_space(12.0);
        right(ui, config);
    } else {
        let column_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.set_width(column_width);
                left(ui, config);
            });
            ui.vertical(|ui| {
                ui.set_width(column_width);
                right(ui, config);
            });
        });
    }
}

pub fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.label(egui::RichText::new(title).strong());
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        content(ui);
    });
    ui.add_space(9.0);
}

fn field<'a>(config: &'a mut Value, section: &str, key: &str) -> &'a mut Value {
    let root = config
        .as_object_mut()
        .expect("settings JSON must be an object");
    let section = root
        .entry(section)
        .or_insert_with(|| Value::Object(Map::new()));
    section
        .as_object_mut()
        .expect("settings section must be an object")
        .entry(key)
        .or_insert(Value::Null)
}

pub fn check(ui: &mut egui::Ui, config: &mut Value, section: &str, key: &str, label: &str) {
    let value = field(config, section, key);
    let mut checked = value.as_bool().unwrap_or(false);
    if ui.checkbox(&mut checked, label).changed() {
        *value = Value::Bool(checked);
    }
}

pub fn text(ui: &mut egui::Ui, config: &mut Value, section: &str, key: &str, label: &str) {
    let value = field(config, section, key);
    let mut current = value.as_str().unwrap_or("").to_owned();
    ui.horizontal(|ui| {
        ui.label(label);
        if ui.text_edit_singleline(&mut current).changed() {
            *value = Value::String(current);
        }
    });
}

pub fn uint(
    ui: &mut egui::Ui,
    config: &mut Value,
    section: &str,
    key: &str,
    label: &str,
    range: std::ops::RangeInclusive<u64>,
) {
    let value = field(config, section, key);
    let mut current = value.as_u64().unwrap_or(0);
    ui.horizontal(|ui| {
        ui.label(label);
        if ui
            .add(egui::DragValue::new(&mut current).range(range))
            .changed()
        {
            *value = Value::from(current);
        }
    });
}

pub fn decimal(
    ui: &mut egui::Ui,
    config: &mut Value,
    section: &str,
    key: &str,
    label: &str,
    range: std::ops::RangeInclusive<f64>,
) {
    let value = field(config, section, key);
    let mut current = value.as_f64().unwrap_or(0.0);
    ui.horizontal(|ui| {
        ui.label(label);
        if ui
            .add(egui::DragValue::new(&mut current).range(range))
            .changed()
        {
            *value = Value::from(current);
        }
    });
}

pub fn choice(
    ui: &mut egui::Ui,
    config: &mut Value,
    section: &str,
    key: &str,
    label: &str,
    options: &[(&str, &str)],
) {
    let value = field(config, section, key);
    let mut current = value.as_str().unwrap_or("").to_owned();
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((section, key))
            .selected_text(
                options
                    .iter()
                    .find(|(_, stored)| *stored == current)
                    .map_or(current.as_str(), |(display, _)| display),
            )
            .show_ui(ui, |ui| {
                for (display, stored) in options {
                    ui.selectable_value(&mut current, (*stored).to_owned(), *display);
                }
            });
    });
    *value = Value::String(current);
}

pub fn index(
    ui: &mut egui::Ui,
    config: &mut Value,
    section: &str,
    key: &str,
    label: &str,
    options: &[&str],
) {
    let value = field(config, section, key);
    let mut current = value.as_u64().unwrap_or(0) as usize;
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((section, key))
            .selected_text(options.get(current).copied().unwrap_or("Unknown"))
            .show_ui(ui, |ui| {
                for (index, option) in options.iter().enumerate() {
                    ui.selectable_value(&mut current, index, *option);
                }
            });
    });
    *value = Value::from(current);
}

pub fn path(ui: &mut egui::Ui, config: &mut Value, section: &str, key: &str, label: &str) {
    let value = field(config, section, key);
    let mut current = value.as_str().unwrap_or("").to_owned();
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(&mut current);
        if ui.button("Browse").clicked() {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                current = folder.to_string_lossy().into_owned();
            }
        }
    });
    *value = Value::String(current);
}
