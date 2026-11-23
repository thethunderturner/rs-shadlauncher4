use super::cache::{Db, LoadedTitle, LoadedTitles};
use crate::config::storage::user_directory;
use crate::gui::windows::progress::{self, Progress};
use eframe::egui;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io::Read, time::Duration};

const RELEASE_ASSET_URL: &str = "https://github.com/shadps4-compatibility/shadps4-game-compatibility/releases/latest/download/compatibility_data.json";

#[derive(Debug, PartialEq, Eq)]
pub struct Compatibility {
    pub status: String,
    pub milestone: Option<String>,
}

pub type Reports = BTreeMap<String, Compatibility>;

#[derive(Deserialize)]
struct Report {
    status: String,
    version: String,
}

pub fn download(mut progress: impl FnMut(Option<f32>, String)) -> Result<Reports, String> {
    progress(None, "Connecting to GitHub".into());
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build()
        .new_agent();
    let mut response = agent
        .get(RELEASE_ASSET_URL)
        .header("User-Agent", "shadLauncher4")
        .call()
        .map_err(|error| format!("Could not download compatibility: {error}"))?;
    let total = response.body().content_length().filter(|&total| total > 0);
    let mut reader = response.body_mut().as_reader();
    let mut bytes = Vec::new();
    let mut buffer = [0; 32 * 1024];
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("Could not read compatibility: {error}"))?;
        if count == 0 {
            break;
        }
        if bytes.len() + count > 10 * 1024 * 1024 {
            return Err("Compatibility download exceeds 10 MB".into());
        }
        bytes.extend_from_slice(&buffer[..count]);
        progress(
            total.map(|total| (bytes.len() as f32 / total as f32).min(1.0)),
            format!("Downloading compatibility: {} KB", bytes.len() / 1024),
        );
    }
    let reports = parse(&bytes, platform())?;
    progress(None, "Saving compatibility_data.json".into());
    let directory = user_directory().map_err(|error| format!("Could not locate user folder: {error}"))?;
    fs::create_dir_all(&directory).map_err(|error| format!("Could not create user folder: {error}"))?;
    let path = directory.join("compatibility_data.json");
    fs::write(&path, &bytes).map_err(|error| format!("Could not save {}: {error}", path.display()))?;
    Ok(reports)
}

pub fn load_local() -> Result<Reports, String> {
    let path = user_directory()
        .map_err(|error| format!("Could not locate user folder: {error}"))?
        .join("compatibility_data.json");
    match fs::read(&path) {
        Ok(bytes) => parse(&bytes, platform()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Reports::new()),
        Err(error) => Err(format!("Could not read {}: {error}", path.display())),
    }
}

fn platform() -> &'static str {
    match std::env::consts::OS {
        "windows" => "os-windows",
        "macos" => "os-macOS",
        "linux" => "os-linux",
        _ => "os-unknown",
    }
}

fn parse(bytes: &[u8], platform: &str) -> Result<Reports, String> {
    let data: BTreeMap<String, BTreeMap<String, Report>> =
        serde_json::from_slice(bytes).map_err(|error| format!("Invalid compatibility data: {error}"))?;
    if data.is_empty() {
        return Err("Compatibility data is empty".into());
    }
    Ok(data
        .into_iter()
        .map(|(serial, mut platforms)| {
            let report = platforms.remove(platform);
            let status = match report.as_ref().map(|report| report.status.as_str()) {
                Some("status-playable") => "Playable",
                Some("status-ingame") => "Ingame",
                Some("status-menus") => "Menus",
                Some("status-boots") => "Boots",
                Some("status-nothing") => "Nothing",
                _ => "Unknown",
            }
            .to_owned();
            let milestone = report
                .map(|report| report.version)
                .filter(|version| !version.is_empty() && !version.eq_ignore_ascii_case("unknown"));
            (serial, Compatibility { status, milestone })
        })
        .collect())
}

pub fn status_color(status: &str) -> egui::Color32 {
    match status {
        "Playable" => egui::Color32::from_rgb(0, 201, 81),
        "Ingame" => egui::Color32::from_rgb(43, 127, 255),
        "Menus" => egui::Color32::from_rgb(142, 81, 255),
        "Boots" => egui::Color32::from_rgb(240, 177, 0),
        "Nothing" => egui::Color32::from_rgb(251, 44, 54),
        _ => egui::Color32::GRAY,
    }
}

pub fn apply(titles: &mut [LoadedTitle], reports: &Reports) {
    for title in titles {
        let report = reports.get(&title.title.serial);
        title.compatibility = report.map(|report| report.status.clone());
        title.title.milestone = report.and_then(|report| report.milestone.clone());
    }
}

#[derive(Default)]
pub struct Updater {
    receiver: Option<std::sync::mpsc::Receiver<UpdateEvent>>,
    progress: Option<Progress>,
    error: Option<String>,
}

enum UpdateEvent {
    Progress(Progress),
    Finished(Result<LoadedTitles, String>),
}

impl Updater {
    pub fn is_running(&self) -> bool {
        self.receiver.is_some()
    }

    pub fn start(&mut self, ctx: &egui::Context) {
        if self.is_running() {
            return;
        }
        self.error = None;
        self.progress = Some(Progress {
            title: "Updating compatibility".into(),
            fraction: None,
            message: "Starting download".into(),
        });
        let (sender, receiver) = std::sync::mpsc::channel();
        let ctx = ctx.clone();
        let worker = std::thread::Builder::new()
            .name("compatibility-update".into())
            .spawn(move || {
                let result = (|| {
                    let reports = download(|fraction, message| {
                        let _ = sender.send(UpdateEvent::Progress(Progress {
                            title: "Updating compatibility".into(),
                            fraction,
                            message,
                        }));
                        ctx.request_repaint();
                    })?;
                    let mut db = Db::connect().map_err(|error| error.to_string())?;
                    db.create_tables().map_err(|error| error.to_string())?;
                    db.save_compatibility(&reports)
                        .map_err(|error| error.to_string())?;
                    LoadedTitles::try_load().map_err(|error| error.to_string())
                })();
                let _ = sender.send(UpdateEvent::Finished(result));
                ctx.request_repaint();
            });
        match worker {
            Ok(_) => self.receiver = Some(receiver),
            Err(error) => {
                self.progress = None;
                self.error = Some(format!("Could not start compatibility update: {error}"));
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<LoadedTitles> {
        let mut completed = None;
        while let Some(receiver) = &self.receiver {
            match receiver.try_recv() {
                Ok(UpdateEvent::Progress(progress)) => self.progress = Some(progress),
                Ok(UpdateEvent::Finished(result)) => {
                    self.receiver = None;
                    self.progress = None;
                    match result {
                        Ok(titles) => {
                            completed = Some(titles);
                        }
                        Err(error) => self.error = Some(error),
                    }
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.receiver = None;
                    self.progress = None;
                    self.error = Some(
                        "The compatibility worker stopped before completing. Try updating again.".into(),
                    );
                    break;
                }
            }
        }

        if let Some(progress) = &self.progress {
            progress::show(ui.ctx(), progress);
            ui.ctx().request_repaint_after(Duration::from_millis(100));
        }
        if let Some(error) = &self.error {
            let mut open = true;
            egui::Window::new("Compatibility update failed")
                .id(egui::Id::unique("compatibility_error"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(error);
                });
            if !open {
                self.error = None
            }
        }
        completed
    }
}
