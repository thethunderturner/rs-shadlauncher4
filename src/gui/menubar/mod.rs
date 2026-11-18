use crate::config::emulator::Config;
use controllers::Controllers;
use eframe::egui;
use file::File;
use help::Help;
use manage::Manage;
use settings::Settings;
use utilities::Utilities;
use view::View;

pub mod controllers;
pub mod file;
pub mod help;
pub mod manage;
pub mod settings;
pub mod utilities;
pub mod view;

#[derive(Default)]
pub struct MenuBar {
    file: File,
    settings: Settings,
    controllers: Controllers,
    manage: Manage,
    utilities: Utilities,
    view: View,
    help: Help,
}

impl MenuBar {
    pub fn show(&mut self, ui: &mut egui::Ui, config: &mut Config) {
        egui::Panel::top("menubar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                self.file.show(ui);
                self.settings.show(ui, config);
                self.controllers.show(ui);
                self.manage.show(ui);
                self.utilities.show(ui);
                self.view.show(ui);
                self.help.show(ui);
            });
        });
    }
}
