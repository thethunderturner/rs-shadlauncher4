use eframe::egui::Color32;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

const RELEASE_ASSET_URL: &str = "https://github.com/shadps4-compatibility/shadps4-game-compatibility/releases/latest/download/compatibility_data.json";

pub struct Compatibility {
    pub status: Status,
}

impl Compatibility {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn color(&self) -> Color32 {
        self.status.color()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    Unknown,
    Nothing,
    Boots,
    Menus,
    Ingame,
    Playable,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: Option<u64>,
}

impl Status {
    pub fn color(self) -> Color32 {
        match self {
            Self::Unknown => Color32::BLACK,
            Self::Nothing => Color32::RED,
            Self::Boots => Color32::YELLOW,
            Self::Menus => Color32::PURPLE,
            Self::Ingame => Color32::BLUE,
            Self::Playable => Color32::GREEN,
        }
    }
}

pub fn refresh(mut on_progress: impl FnMut(DownloadProgress)) -> Result<PathBuf, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build()
        .into();
    let mut response = agent
        .get(RELEASE_ASSET_URL)
        .call()
        .map_err(|error| format!("Could not download compatibility data: {error}"))?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    let mut bytes = Vec::new();
    let mut buffer = [0; 32 * 1024];
    let mut progress = DownloadProgress { downloaded: 0, total };
    on_progress(progress);
    let mut reader = response.body_mut().as_reader();
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("Could not read compatibility data: {error}"))?;
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..count]);
        progress.downloaded += count as u64;
        on_progress(progress);
    }
    let path = crate::common::user_directory()
        .map_err(|error| format!("Could not locate user directory: {error}"))?
        .join("compatibility_data.json");
    store(&path, &bytes)?;
    Ok(path)
}

fn store(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("Downloaded compatibility data is not valid JSON: {error}"))?;
    if !value.is_object() {
        return Err("Downloaded compatibility data has an unexpected format.".to_owned());
    }
    let directory = path
        .parent()
        .ok_or("Compatibility data path has no parent directory.")?;
    let mut temporary = tempfile::NamedTempFile::new_in(directory)
        .map_err(|error| format!("Could not create temporary compatibility file: {error}"))?;
    temporary
        .write_all(bytes)
        .map_err(|error| format!("Could not write compatibility data: {error}"))?;
    temporary
        .persist(path)
        .map_err(|error| format!("Could not replace compatibility data: {error}"))?;
    Ok(())
}
