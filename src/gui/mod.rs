use crate::compatibility::DownloadProgress;
use crate::gui::centralpanel::Centralpanel;
use crate::gui::menubar::Menubar;
use crate::gui::sidepanel::Sidepanel;
use crate::gui::toolbar::Toolbar;
use crate::scanning;
use eframe::egui;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant};

pub mod centralpanel;
pub mod menubar;
pub mod sidepanel;
pub mod toolbar;

enum CompatibilityDownloadEvent {
    Progress(DownloadProgress),
    Finished(Result<(), String>),
}

#[derive(Default)]
struct CompatibilityDownloadWindow {
    progress: Option<DownloadProgress>,
    error: Option<String>,
}

pub struct LauncherApp {
    pub menubar: Menubar,
    pub sidepanel: Sidepanel,
    pub centralpanel: Centralpanel,
    toolbar: Toolbar,
    scan_rx: Option<Receiver<Result<(Vec<scanning::Title>, Duration), ()>>>,
    scan_pending: bool,
    compatibility_rx: Option<Receiver<CompatibilityDownloadEvent>>,
    compatibility_window: Option<CompatibilityDownloadWindow>,
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut app = Self {
            menubar: Menubar::default(),
            sidepanel: Sidepanel::default(),
            centralpanel: Centralpanel::default(),
            toolbar: Toolbar::default(),
            scan_rx: None,
            scan_pending: false,
            compatibility_rx: None,
            compatibility_window: None,
        };
        app.sidepanel.load_paths();
        app.start_scan(&cc.egui_ctx);
        app
    }

    fn start_scan(&mut self, ctx: &egui::Context) {
        if self.scan_rx.is_some() {
            return;
        }
        if !self.sidepanel.can_refresh() {
            return;
        }
        let games_paths = self.sidepanel.games_paths().to_vec();
        let (tx, scan_rx) = mpsc::channel();
        let ctx = ctx.clone();
        self.centralpanel.start_scan();
        self.scan_rx = Some(scan_rx);
        std::thread::spawn(move || {
            let titles = std::panic::catch_unwind(|| {
                let started = Instant::now();
                let titles = games_paths.iter().flat_map(|path| scanning::scan(path)).collect();
                (titles, started.elapsed())
            })
            .map_err(|_| ());
            let _ = tx.send(titles);
            ctx.request_repaint();
        });
    }
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut download_finished = false;
        if let Some(receiver) = &self.compatibility_rx {
            loop {
                match receiver.try_recv() {
                    Ok(CompatibilityDownloadEvent::Progress(progress)) => {
                        if let Some(window) = &mut self.compatibility_window {
                            window.progress = Some(progress);
                        }
                    }
                    Ok(CompatibilityDownloadEvent::Finished(Ok(()))) => {
                        self.compatibility_window = None;
                        download_finished = true;
                        break;
                    }
                    Ok(CompatibilityDownloadEvent::Finished(Err(error))) => {
                        if let Some(window) = &mut self.compatibility_window {
                            window.error = Some(error);
                        }
                        download_finished = true;
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        if let Some(window) = &mut self.compatibility_window {
                            window.error = Some("Compatibility download stopped unexpectedly.".to_owned());
                        }
                        download_finished = true;
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }
        }
        if download_finished {
            self.compatibility_rx = None;
        }
        if let Some(scan_rx) = &self.scan_rx {
            match scan_rx.try_recv() {
                Ok(Ok((titles, duration))) => {
                    self.centralpanel.set_titles(titles, duration);
                    self.scan_rx = None;
                }
                Ok(Err(())) | Err(TryRecvError::Disconnected) => {
                    self.centralpanel.set_scan_failed();
                    self.scan_rx = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }

        if self.scan_pending && self.scan_rx.is_none() {
            self.scan_pending = false;
            self.start_scan(ui.ctx());
        }

        self.menubar.show(ui);
        let actions = self.sidepanel.show(
            ui,
            self.scan_rx.is_some(),
            self.sidepanel.can_refresh(),
            self.compatibility_rx.is_some(),
        );
        if let Some(search) = actions.search {
            self.centralpanel.set_search(search);
        }
        if let Some(filter) = actions.library_filter {
            self.centralpanel.set_library_filter(filter);
        }
        let refresh_compatibility = actions.refresh_compatibility;
        if actions.refresh_list {
            self.start_scan(ui.ctx());
        }
        if refresh_compatibility && self.compatibility_rx.is_none() {
            self.compatibility_window = Some(CompatibilityDownloadWindow::default());
            let (tx, receiver) = mpsc::channel();
            let ctx = ui.ctx().clone();
            self.compatibility_rx = Some(receiver);
            std::thread::spawn(move || {
                let result = crate::compatibility::refresh(|progress| {
                    let _ = tx.send(CompatibilityDownloadEvent::Progress(progress));
                    ctx.request_repaint();
                });
                let _ = tx.send(CompatibilityDownloadEvent::Finished(result.map(|_| ())));
                ctx.request_repaint();
            });
        }
        if let Some(window) = &self.compatibility_window {
            let mut close = false;
            egui::Window::new("Refresh Compatibility")
                .id(egui::Id::new("compatibility_download_window"))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(320.0);
                    if let Some(error) = &window.error {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                        if ui.button("Close").clicked() {
                            close = true;
                        }
                    } else {
                        ui.label("Downloading compatibility data…");
                        let fraction = window.progress.and_then(|progress| {
                            progress
                                .total
                                .filter(|&total| total > 0)
                                .map(|total| (progress.downloaded as f32 / total as f32).clamp(0.0, 1.0))
                        });
                        let bar = egui::ProgressBar::new(fraction.unwrap_or(0.0))
                            .desired_width(ui.available_width())
                            .animate(fraction.is_none());
                        ui.add(if fraction.is_some() {
                            bar.show_percentage()
                        } else {
                            bar
                        });
                        if let Some(progress) = window.progress {
                            if let Some(total) = progress.total {
                                ui.label(format!(
                                    "{:.1} / {:.1} MiB",
                                    progress.downloaded as f64 / 1_048_576.0,
                                    total as f64 / 1_048_576.0,
                                ));
                            } else {
                                ui.label(format!(
                                    "Downloaded {:.1} MiB",
                                    progress.downloaded as f64 / 1_048_576.0,
                                ));
                            }
                        }
                    }
                });
            if close {
                self.compatibility_window = None;
            }
        }
        self.toolbar.show(ui);
        self.centralpanel.show(ui);

        if self.menubar.show_windows(ui.ctx()) && self.sidepanel.load_paths() {
            if self.sidepanel.can_refresh() {
                self.scan_pending = true;
            } else {
                self.scan_rx = None;
                self.scan_pending = false;
                self.centralpanel.clear_titles();
            }
        }
    }
}
