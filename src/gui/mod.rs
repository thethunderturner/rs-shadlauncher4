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
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            menubar: Menubar::default(),
            sidepanel: Sidepanel::default(),
            centralpanel: Centralpanel::default(),
            toolbar: Toolbar::default(),
            scan_rx: None,
        }
    }

    fn start_scan(&mut self, ctx: &egui::Context) {
        if self.scan_rx.is_some() {
            return;
        }
        let Some(games_path) = self.toolbar.games_path().map(|path| path.to_path_buf()) else {
            return;
        };
        let (tx, scan_rx) = mpsc::channel();
        let ctx = ctx.clone();
        self.centralpanel.start_scan(&ctx);
        self.scan_rx = Some(scan_rx);
        std::thread::spawn(move || {
            let titles = std::panic::catch_unwind(|| {
                let started = Instant::now();
                let titles = scanning::scan(&games_path);
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

        self.menubar.show(ui);
        let actions = self
            .sidepanel
            .show(ui, self.scan_rx.is_some(), self.toolbar.can_refresh());
        if let Some(search) = actions.search {
            self.centralpanel.set_search(search);
        }
        if actions.refresh_list {
            self.start_scan(ui.ctx());
        }
        if self.toolbar.show(ui, self.scan_rx.is_some()) {
            self.start_scan(ui.ctx());
        }
        self.centralpanel.show(ui);

        self.menubar.show_windows(ui.ctx());
    }
}
