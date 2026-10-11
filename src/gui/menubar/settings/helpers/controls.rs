use eframe::egui;
use std::path::PathBuf;

pub fn two_columns<T>(
    ui: &mut egui::Ui,
    config: &mut T,
    left: impl FnOnce(&mut egui::Ui, &mut T),
    right: impl FnOnce(&mut egui::Ui, &mut T),
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

pub fn check(ui: &mut egui::Ui, value: &mut bool, label: &str) {
    ui.checkbox(value, label);
}

pub fn text(ui: &mut egui::Ui, value: &mut String, label: &str) {
    ui.vertical(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value);
    });
}

pub fn uint<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut T,
    label: &str,
    range: std::ops::RangeInclusive<T>,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).range(range));
    });
}

pub fn decimal(ui: &mut egui::Ui, value: &mut f64, label: &str, range: std::ops::RangeInclusive<f64>) {
    uint(ui, value, label, range);
}

pub fn choice(ui: &mut egui::Ui, value: &mut String, id: &str, label: &str, options: &[(&str, &str)]) {
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(id)
            .selected_text(
                options
                    .iter()
                    .find(|(_, stored)| *stored == value)
                    .map_or(value.as_str(), |(display, _)| display),
            )
            .show_ui(ui, |ui| {
                for (display, stored) in options {
                    ui.selectable_value(value, (*stored).to_owned(), *display);
                }
            });
    });
}

pub fn index(ui: &mut egui::Ui, value: &mut u32, id: &str, label: &str, options: &[&str]) {
    ui.vertical(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(id)
            .selected_text(options.get(*value as usize).copied().unwrap_or("Unknown"))
            .show_ui(ui, |ui| {
                for (index, option) in options.iter().enumerate() {
                    ui.selectable_value(value, index as u32, *option);
                }
            });
    });
}

pub fn path(ui: &mut egui::Ui, value: &mut String) {
    ui.vertical(|ui| {
        ui.text_edit_singleline(value);
        if ui.button("Browse").clicked() {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                *value = folder.to_string_lossy().into_owned();
            }
        }
    });
}

pub fn path_buf(ui: &mut egui::Ui, value: &mut PathBuf) {
    let mut text = value.to_string_lossy().into_owned();
    path(ui, &mut text);
    *value = PathBuf::from(text);
}
