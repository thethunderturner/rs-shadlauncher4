pub(super) mod controllers;
mod emulation;
mod file;
mod help;
mod manage;
mod settings;
mod utilities;
mod view;

use crate::gui::menubar::controllers::Controllers;
use crate::gui::menubar::emulation::Emulation;
use crate::gui::menubar::file::File;
use crate::gui::menubar::help::Help;
use crate::gui::menubar::manage::Manage;
use crate::gui::menubar::settings::Settings;
use crate::gui::menubar::utilities::Utilities;
use crate::gui::menubar::view::View;
use eframe::egui;

pub struct Menubar {
    pub controllers: Controllers,
    pub emulation: Emulation,
    pub file: File,
    pub help: Help,
    pub manage: Manage,
    pub settings: Settings,
    pub utilities: Utilities,
    pub view: View,
}

impl Default for Menubar {
    fn default() -> Self {
        Self {
            controllers: Controllers {
                show_controllers: false,
            },
            emulation: Emulation {},
            file: File {},
            help: Help { show_about: false },
            manage: Manage {},
            settings: Settings {},
            utilities: Utilities,
            view: View {},
        }
    }
}

impl Menubar {
    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                self.file.show(ui);
                self.emulation.show(ui);
                self.settings.show(ui);
                self.controllers.show(ui);
                self.manage.show(ui);
                self.utilities.show(ui);
                self.view.show(ui);
                self.help.show(ui);
            });
        });
    }

    pub(super) fn show_windows(&mut self, ctx: &egui::Context) {
        self.controllers.show_window(ctx);
        self.help.show_window(ctx);
    }
}
