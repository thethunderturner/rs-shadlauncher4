use super::scan::Title;
use crate::config::storage::user_directory;
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
    progress(None, "Saving compatibility_data.json".into());
    let directory = user_directory().map_err(|error| format!("Could not locate user folder: {error}"))?;
    fs::create_dir_all(&directory).map_err(|error| format!("Could not create user folder: {error}"))?;
    let path = directory.join("compatibility_data.json");
    fs::write(&path, &bytes).map_err(|error| format!("Could not save {}: {error}", path.display()))?;
    drop(bytes);

    progress(None, "Reading compatibility_data.json".into());
    let bytes = fs::read(&path).map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    let platform = match std::env::consts::OS {
        "windows" => "os-windows",
        "macos" => "os-macOS",
        "linux" => "os-linux",
        _ => "os-unknown",
    };
    parse(&bytes, platform)
}

fn parse(bytes: &[u8], platform: &str) -> Result<Reports, String> {
    let data: BTreeMap<String, BTreeMap<String, Report>> =
        serde_json::from_slice(bytes).map_err(|error| format!("Invalid compatibility data: {error}"))?;
    if data.is_empty() {
        return Err("Compatibility download is empty".into());
    }
    Ok(data
        .into_iter()
        .map(|(serial, mut platforms)| {
            let report = platforms
                .remove(platform)
                .or_else(|| platforms.remove("os-unknown"));
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

pub fn apply(titles: &mut [Title], reports: &Reports) {
    for title in titles {
        let report = reports.get(&title.serial);
        title.compatibility = report.map(|report| report.status.clone());
        title.milestone = report.and_then(|report| report.milestone.clone());
    }
}
