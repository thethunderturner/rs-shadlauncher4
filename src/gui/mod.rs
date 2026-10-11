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

pub struct LauncherApp {
    pub menubar: Menubar,
    pub sidepanel: Sidepanel,
    pub centralpanel: Centralpanel,
    toolbar: Toolbar,
    scan_rx: Option<Receiver<Result<(Vec<scanning::Title>, Duration), ()>>>,
    scan_pending: bool,
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
        let actions = self
            .sidepanel
            .show(ui, self.scan_rx.is_some(), self.sidepanel.can_refresh());
        if let Some(search) = actions.search {
            self.centralpanel.set_search(search);
        }
        if let Some(filter) = actions.library_filter {
            self.centralpanel.set_library_filter(filter);
        }
        if actions.refresh_list {
            self.start_scan(ui.ctx());
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
