use crate::app::State;
use crate::config::emulator::Config;
use crate::config::launcher::Launcher;
use debug::DebugWindow;
use eframe::egui;
use experimental::ExperimentalWindow;
use frontend::FrontendWindow;
use general::GeneralWindow;
use graphics::GraphicsWindow;
use input::InputWindow;
use log::LogWindow;
use network::NetworkWindow;
use paths::PathsWindow;
use user::UserWindow;

pub mod debug;
pub mod experimental;
pub mod frontend;
pub mod general;
pub mod graphics;
pub mod input;
pub mod log;
pub mod network;
pub mod paths;
pub mod user;

pub struct SettingsWindow {
    pub open: bool,
    draft: State,
    custom_serial: Option<String>,
    save_error: Option<String>,
    selected_tab: String,
    general_window: GeneralWindow,
    frontend_window: FrontendWindow,
    graphics_window: GraphicsWindow,
    user_window: UserWindow,
    input_window: InputWindow,
    paths_window: PathsWindow,
    log_window: LogWindow,
    debug_window: DebugWindow,
    network_window: NetworkWindow,
    experimental_window: ExperimentalWindow,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        let config = Config::default();
        let frontend = Launcher::default();
        let selected_tab = frontend.home_tab.clone();
        let general_window = GeneralWindow::new(config.audio.audio_backend);
        let graphics_window = GraphicsWindow::new();
        let input_window = InputWindow::new();
        Self {
            open: false,
            draft: State {
                config,
                launcher: frontend,
                ..State::default()
            },
            custom_serial: None,
            save_error: None,
            selected_tab,
            general_window,
            frontend_window: FrontendWindow::default(),
            graphics_window: graphics_window.expect("REASON"),
            user_window: UserWindow::default(),
            input_window: input_window.expect("REASON"),
            paths_window: PathsWindow::default(),
            log_window: LogWindow::default(),
            debug_window: DebugWindow::default(),
            network_window: NetworkWindow::default(),
            experimental_window: ExperimentalWindow::default(),
        }
    }
}

impl SettingsWindow {
    pub fn begin_edit(&mut self, state: &State) {
        self.custom_serial = None;
        self.save_error = None;
        self.draft.config = state.config.clone();
        self.draft.launcher = state.launcher.clone();
        self.selected_tab = state.launcher.home_tab.clone();
        self.general_window = GeneralWindow::new(state.config.audio.audio_backend);
    }

    pub fn begin_custom_edit(&mut self, serial: &str, state: &State, config: Config) {
        self.begin_edit(state);
        self.general_window = GeneralWindow::new(config.audio.audio_backend);
        self.draft.config = config;
        self.custom_serial = Some(serial.to_owned());
        self.selected_tab = "General".into();
    }

    pub fn select_tab(&mut self, tab: &str) {
        self.selected_tab = tab.to_owned();
    }

    fn save_current(&self) -> std::io::Result<()> {
        match &self.custom_serial {
            Some(serial) => self.draft.config.save_custom(serial),
            None => {
                self.draft.config.save()?;
                self.draft.launcher.save()
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State, is_custom: bool) -> bool {
        if !self.open {
            return false;
        }

        let mut saved = false;
        let window_title = self
            .custom_serial
            .as_ref()
            .map(|serial| format!("Game Settings - {serial}"))
            .unwrap_or_else(|| "Settings".into());
        egui::Window::new(window_title)
            .id(egui::Id::unique(("settings_window", &self.custom_serial)))
            .default_size(egui::vec2(800.0, 500.0))
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .add(egui::Button::new("General").selected("General" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("General");
                            }
                            if ! is_custom // hide for experimental configs
                                && ui
                                .add(egui::Button::new("Frontend").selected("Frontend" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Frontend");
                            }
                            if ui
                                .add(egui::Button::new("Graphics").selected("Graphics" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Graphics");
                            }
                            if ui
                                .add(egui::Button::new("User").selected("User" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("User");
                            }
                            if ui
                                .add(egui::Button::new("Input").selected("Input" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Input");
                            }
                            if ui
                                .add(egui::Button::new("Network").selected("Network" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Network");
                            }
                            if ui
                                .add(egui::Button::new("Paths").selected("Paths" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Paths");
                            }
                            if ui
                                .add(egui::Button::new("Log").selected("Log" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Log");
                            }
                            if ui
                                .add(egui::Button::new("Debug").selected("Debug" == self.selected_tab))
                                .clicked()
                            {
                                self.selected_tab = String::from("Debug");
                            }
                            if is_custom
                                && ui
                                    .add(
                                        egui::Button::new("Experimental")
                                            .selected("Experimental" == self.selected_tab),
                                    )
                                    .clicked()
                            {
                                self.selected_tab = String::from("Experimental");
                            }
                        });
                        ui.separator();
                        match self.selected_tab.as_str() {
                            "General" => self.general_window.show(ui, &mut self.draft.config),
                            "Frontend" => self.frontend_window.show(ui, &mut self.draft),
                            "Graphics" => self.graphics_window.show(ui, &mut self.draft.config),
                            "User" => self.user_window.show(ui, &mut self.draft.config),
                            "Input" => self.input_window.show(ui, &mut self.draft.config),
                            "Network" => self.network_window.show(ui, &mut self.draft.config),
                            "Paths" => self.paths_window.show(ui, &mut self.draft.config),
                            "Log" => self.log_window.show(ui, &mut self.draft.config),
                            "Debug" => self.debug_window.show(ui, &mut self.draft.config),
                            "Experimental" => self.experimental_window.show(ui, &mut self.draft.config),
                            _ => {}
                        }
                        if let Some(error) = &self.save_error {
                            ui.colored_label(ui.visuals().error_fg_color, error);
                        }
                        egui::Panel::bottom("footer").show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                                    let color = ui.visuals().selection;
                                    if ui
                                        .add(
                                            egui::Button::image_and_text(
                                                egui::include_image!("../../../assets/save.svg"),
                                                egui::RichText::new("Save").color(color.stroke.color),
                                            )
                                            .fill(color.bg_fill),
                                        )
                                        .clicked()
                                    {
                                        match self.save_current() {
                                            Ok(()) => {
                                                saved = true;
                                                self.save_error = None;
                                                if let Some(serial) = &self.custom_serial {
                                                    if let Some(loaded) = state
                                                        .loaded_titles
                                                        .loaded_titles
                                                        .iter_mut()
                                                        .find(|loaded| &loaded.title.serial == serial)
                                                    {
                                                        loaded.has_custom_config = true;
                                                    }
                                                }
                                                if self.custom_serial.is_none() {
                                                    state.config = self.draft.config.clone();
                                                    state.launcher = self.draft.launcher.clone();
                                                }
                                                self.open = false;
                                            }
                                            Err(err) => {
                                                self.save_error =
                                                    Some(format!("Failed to save configuration: {err}"))
                                            }
                                        }
                                    };
                                    if ui
                                        .add(egui::Button::image_and_text(
                                            egui::include_image!("../../../assets/check.svg"),
                                            egui::RichText::new("Apply").color(color.stroke.color),
                                        ))
                                        .clicked()
                                    {
                                        match self.save_current() {
                                            Ok(()) => {
                                                saved = true;
                                                self.save_error = None;
                                                if let Some(serial) = &self.custom_serial {
                                                    if let Some(loaded) = state
                                                        .loaded_titles
                                                        .loaded_titles
                                                        .iter_mut()
                                                        .find(|loaded| &loaded.title.serial == serial)
                                                    {
                                                        loaded.has_custom_config = true;
                                                    }
                                                }
                                                if self.custom_serial.is_none() {
                                                    state.config = self.draft.config.clone();
                                                    state.launcher = self.draft.launcher.clone();
                                                }
                                            }
                                            Err(err) => {
                                                self.save_error =
                                                    Some(format!("Failed to apply configuration: {err}"))
                                            }
                                        }
                                    }
                                    if ui
                                        .add(egui::Button::image_and_text(
                                            egui::include_image!("../../../assets/cancel.svg"),
                                            egui::RichText::new("Cancel").color(color.stroke.color),
                                        ))
                                        .clicked()
                                    {
                                        self.open = false;
                                        self.draft.config = state.config.clone();
                                        self.draft.launcher = state.launcher.clone();
                                    }
                                });
                            });
                        });
                    });
                });
            });
        saved
    }
}
