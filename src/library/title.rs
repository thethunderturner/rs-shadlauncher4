use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct Title {
    pub parent_dir: PathBuf,
    pub icon_path: Option<PathBuf>,
    pub background_path: Option<PathBuf>,
    pub name: String,
    pub serial: String,
    pub publisher_id: String,
    pub version: String,
    pub fw: u32,
    pub app_type: Option<u32>,
    pub category: String,
    pub app_path: Option<PathBuf>,
    pub patch_path: Option<PathBuf>,
    pub supports_vr: bool,
    pub supports_neo: bool,
    pub milestone: Option<String>,
}

pub fn existing_file(path: PathBuf) -> Option<PathBuf> {
    path.is_file().then_some(path)
}

pub fn find_icon(game_dir: &Path) -> Option<PathBuf> {
    ["icon0.png", "icon1.png"]
        .into_iter()
        .find_map(|name| existing_file(game_dir.join("sce_sys").join(name)))
}

pub fn find_background(game_dir: &Path) -> Option<PathBuf> {
    ["pic0.png", "pic1.png"]
        .into_iter()
        .find_map(|name| existing_file(game_dir.join("sce_sys").join(name)))
}
