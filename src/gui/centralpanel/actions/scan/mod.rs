use crate::gui::centralpanel::{Centralpanel, actions};
use eframe::egui;
use std::time::Duration;

pub struct Scan {
    pub scan_failed: bool,
    pub scan_duration: Option<Duration>,
}

impl Default for Scan {
    fn default() -> Self {
        Self {
            scan_failed: false,
            scan_duration: None,
        }
    }
}

impl Centralpanel {
    pub fn start_scan(&mut self, ctx: &egui::Context) {
        actions::assets::clear_cache(ctx, &self.titles);
        self.loading = true;
        self.scan.scan_failed = false;
        self.scan.scan_duration = None;
    }

    pub fn set_scan_failed(&mut self) {
        self.loading = false;
        self.scan.scan_failed = true;
    }
}
