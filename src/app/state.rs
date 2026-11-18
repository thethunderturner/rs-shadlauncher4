use crate::library::{
    cache::Db,
    compatibility::{self, Reports},
    scan::{Scan, Title},
};
use eframe::egui;
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
};

#[derive(Clone)]
pub struct Progress {
    pub title: &'static str,
    pub fraction: Option<f32>,
    pub message: String,
}

#[derive(Default)]
pub struct AppState {
    pub titles: Vec<Title>,
    pub load_ms: f64,
    pub progress: Option<Arc<Mutex<Progress>>>,
    pub error: Option<String>,
    pub compatibility_message: Option<String>,
    pending_compatibility: Option<Receiver<Result<Reports, String>>>,
    pending: Option<Receiver<Result<(Vec<Title>, f64), String>>>,
}

impl AppState {
    fn start_progress(&mut self, title: &'static str) -> Arc<Mutex<Progress>> {
        let progress = Arc::new(Mutex::new(Progress {
            title,
            fraction: None,
            message: "Starting".into(),
        }));
        self.progress = Some(progress.clone());
        progress
    }

    pub fn is_busy(&self) -> bool {
        self.is_loading() || self.pending_compatibility.is_some()
    }

    pub fn refresh_compatibility(&mut self, ctx: &egui::Context) {
        if self.is_busy() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.pending_compatibility = Some(rx);
        self.compatibility_message = None;
        let progress = self.start_progress("Refresh Compatibility");
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<Reports, String> {
                    let report = |fraction, message| update_progress(&progress, &ctx, fraction, message);
                    let reports = compatibility::download(report)?;
                    report(None, "Saving compatibility to the cache".into());
                    let mut db = Db::connect().map_err(|error| error.to_string())?;
                    db.create_tables().map_err(|error| error.to_string())?;
                    db.save_compatibility(&reports)
                        .map_err(|error| error.to_string())?;
                    Ok(reports)
                }))
                .unwrap_or_else(|_| Err("Compatibility worker failed.".into()));
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }

    pub fn is_loading(&self) -> bool {
        self.pending.is_some()
    }

    pub fn load(&mut self, ctx: &egui::Context, refresh: bool) {
        if self.is_busy() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.error = None;
        let progress = refresh.then(|| self.start_progress("Scan Library"));
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<(Vec<Title>, f64), String> {
                    if let Some(progress) = &progress {
                        Scan::scan(|fraction, message| update_progress(progress, &ctx, fraction, message));
                    }
                    let started = std::time::Instant::now();
                    let db = Db::connect().map_err(|e| e.to_string())?;
                    db.create_tables().map_err(|e| e.to_string())?;
                    let titles = db.load_titles().map_err(|e| e.to_string())?;
                    Ok((titles, started.elapsed().as_secs_f64() * 1000.0))
                },
            ))
            .unwrap_or_else(|_| Err("Library worker failed. Check the console for details.".into()));
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }

    pub fn poll(&mut self, ctx: &egui::Context) -> bool {
        if let Some(rx) = &self.pending_compatibility {
            let result = match rx.try_recv() {
                Ok(result) => result,
                Err(mpsc::TryRecvError::Empty) => return false,
                Err(mpsc::TryRecvError::Disconnected) => Err("Compatibility worker stopped.".into()),
            };
            self.pending_compatibility = None;
            self.progress = None;
            return match result {
                Ok(reports) => {
                    let started = std::time::Instant::now();
                    compatibility::apply(&mut self.titles, &reports);
                    self.load_ms = started.elapsed().as_secs_f64() * 1000.0;
                    self.compatibility_message = None;
                    true
                }
                Err(error) => {
                    self.compatibility_message = Some(format!("Compatibility refresh failed: {error}"));
                    false
                }
            };
        }
        let Some(rx) = &self.pending else {
            return false;
        };
        let result = match rx.try_recv() {
            Ok(result) => result,
            Err(mpsc::TryRecvError::Empty) => return false,
            Err(mpsc::TryRecvError::Disconnected) => Err("Library worker stopped.".into()),
        };
        self.pending = None;
        self.progress = None;
        match result {
            Ok((titles, load_ms)) => {
                self.load_ms = load_ms;
                for title in &self.titles {
                    ctx.forget_image(&format!("bytes://title-icon/{}", title.serial));
                }
                self.titles = titles;
                true
            }
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }
}

fn update_progress(progress: &Mutex<Progress>, ctx: &egui::Context, fraction: Option<f32>, message: String) {
    if let Ok(mut progress) = progress.lock() {
        progress.fraction = fraction;
        progress.message = message;
    }
    ctx.request_repaint();
}
