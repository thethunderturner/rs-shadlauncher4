mod configuration;
mod emulation;
mod file;
mod help;
mod manage;
mod utilities;
mod view;

use eframe::egui;

pub(super) fn show(ui: &mut egui::Ui, show_about: &mut bool) {
    egui::Panel::top("menu_bar").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            file::show(ui);
            emulation::show(ui);
            configuration::show(ui);
            manage::show(ui);
            utilities::show(ui);
            view::show(ui);
            help::show(ui, show_about);
        });
    });
}
