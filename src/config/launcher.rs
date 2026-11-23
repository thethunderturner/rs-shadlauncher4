use crate::config::storage::user_directory;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Launcher {
    pub show_background_image: bool,
    pub background_image_opacity: u32,
    pub play_background_music: bool,
    pub background_music_volume: u32,
    pub show_compatibility: bool,
    pub check_compatibility_at_startup: bool,
    pub check_for_updates: bool,
    pub update_channel: crate::launcher::update::Channel,
    pub show_change_log: bool,
    pub home_tab: String,
    pub version_path: PathBuf,
    pub pkg_install_dir: PathBuf,
    pub localization: u32,
    pub theme: u32,
}

impl Launcher {
    pub fn save(&self) -> io::Result<()> {
        let directory = user_directory()?;
        fs::create_dir_all(&directory)?;
        let bytes = serde_json::to_vec_pretty(self).map_err(io::Error::other)?;
        fs::write(directory.join("launcher.json"), bytes)
    }

    pub fn load() -> Self {
        let result = (|| -> io::Result<Self> {
            let path = user_directory()?.join("launcher.json");
            serde_json::from_slice(&fs::read(path)?).map_err(io::Error::other)
        })();

        result.unwrap_or_else(|error| {
            eprintln!("Failed to load configuration: {error}");
            Self::default()
        })
    }
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            show_background_image: false,
            background_image_opacity: 15,
            play_background_music: false,
            background_music_volume: 5,
            show_compatibility: false,
            check_compatibility_at_startup: false,
            check_for_updates: false,
            update_channel: crate::launcher::update::Channel::default(),
            show_change_log: false,
            home_tab: "General".to_owned(),
            version_path: user_directory().unwrap().join("versions"),
            pkg_install_dir: PathBuf::new(),
            localization: 1, // English US
            theme: 1,        // Dark
        }
    }
}
