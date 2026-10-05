use crate::gui::centralpanel::Centralpanel;
use crate::gui::menubar::Menubar;
use crate::gui::sidepanel::Sidepanel;
use crate::scanning;
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError};

pub mod centralpanel;
pub mod menubar;
pub mod sidepanel;
pub mod toolbar;

pub struct LauncherApp {
    pub menubar: Menubar,
    pub sidepanel: Sidepanel,
    pub centralpanel: Centralpanel,
    scan_rx: Option<Receiver<Result<Vec<scanning::Title>, ()>>>,
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, games_path: PathBuf) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let (tx, scan_rx) = mpsc::channel();
        let ctx = cc.egui_ctx.clone();
        std::thread::spawn(move || {
            let titles = std::panic::catch_unwind(|| scanning::scan(&games_path)).map_err(|_| ());
            let _ = tx.send(titles);
            ctx.request_repaint();
        });

        Self {
            menubar: Menubar::default(),
            sidepanel: Sidepanel::default(),
            centralpanel: Centralpanel::default(),
            scan_rx: Some(scan_rx),
        }
    }
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(scan_rx) = &self.scan_rx {
            match scan_rx.try_recv() {
                Ok(Ok(titles)) => {
                    self.centralpanel.set_titles(titles);
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
        if let Some(search) = self.sidepanel.show(ui) {
            self.centralpanel.set_search(search);
        }
        self.centralpanel.show(ui);

        self.menubar.show_windows(ui.ctx());
    }
}
