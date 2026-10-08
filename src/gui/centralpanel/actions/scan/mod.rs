use crate::gui::centralpanel::Centralpanel;
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
    pub fn start_scan(&mut self) {
        self.actions.row.stop_audio();
        self.actions.assets.clear_cache();
        self.loading = true;
        self.scan.scan_failed = false;
        self.scan.scan_duration = None;
    }

    pub fn set_scan_failed(&mut self) {
        self.loading = false;
        self.scan.scan_failed = true;
    }
}
