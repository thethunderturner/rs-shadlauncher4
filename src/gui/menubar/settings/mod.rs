pub mod windows;

use eframe::egui;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum SettingsTab {
    #[default]
    General,
    Audio,
    Gui,
    Input,
    Paths,
    Log,
    Debug,
    Experimental,
}

impl SettingsTab {
    const ALL: [Self; 8] = [
        Self::General,
        Self::Audio,
        Self::Gui,
        Self::Input,
        Self::Paths,
        Self::Log,
        Self::Debug,
        Self::Experimental,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Audio => "Audio",
            Self::Gui => "GUI",
            Self::Input => "Input",
            Self::Paths => "Paths",
            Self::Log => "Log",
            Self::Debug => "Debug",
            Self::Experimental => "Experimental",
        }
    }

    fn icon(self) -> egui::ImageSource<'static> {
        match self {
            Self::General => egui::include_image!("../../../assets/menubar/settings/general.svg"),
            Self::Audio => egui::include_image!("../../../assets/menubar/settings/audio.svg"),
            Self::Gui => egui::include_image!("../../../assets/menubar/settings/window.svg"),
            Self::Input => egui::include_image!("../../../assets/menubar/settings/input.svg"),
            Self::Paths => egui::include_image!("../../../assets/menubar/settings/paths.svg"),
            Self::Log => egui::include_image!("../../../assets/menubar/settings/log.svg"),
            Self::Debug => egui::include_image!("../../../assets/menubar/settings/debug.svg"),
            Self::Experimental => {
                egui::include_image!("../../../assets/menubar/settings/experimental.svg")
            }
        }
    }
}

#[derive(Default)]
pub struct Settings {
    open: bool,
    selected_tab: SettingsTab,
}

impl Settings {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Settings", |ui| {
            for tab in SettingsTab::ALL {
                if ui
                    .add(
                        egui::Button::image_and_text(tab.icon(), tab.label())
                            .image_tint_follows_text_color(true),
                    )
                    .clicked()
                {
                    self.selected_tab = tab;
                    self.open = true;
                    ui.close();
                }
            }
        });
    }

    pub(super) fn show_window(&mut self, ctx: &egui::Context) {
        let selected_tab = &mut self.selected_tab;
        egui::Window::new("Settings")
            .id(egui::Id::new("settings_window"))
            .open(&mut self.open)
            .default_size([640.0, 400.0])
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for tab in SettingsTab::ALL {
                        ui.selectable_value(selected_tab, tab, tab.label());
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("settings_content")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label("No settings available in this category yet.");
                    });
            });
    }
}
