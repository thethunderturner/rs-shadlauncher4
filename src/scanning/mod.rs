use crate::sfo::internal::data_table::SfoDataTable;
use crate::sfo::parse_param::attribute;
use crate::sfo::parse_param::attribute::{supports_neo, supports_vr};
use crate::sfo::reader::read_sfo;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct Title {
    pub icon_path: Option<PathBuf>,
    pub background_path: Option<PathBuf>,
    pub parent_dir: PathBuf,
    pub serial: String,
    pub compatibility: Option<String>,
    pub publisher_id: String,
    pub playtime: String,
    pub supports_vr: bool,
    pub supports_neo: bool,
    pub app: App,
    pub patch: Option<Patch>, // Maybe a title doesn't have a patch
}

#[derive(Clone, Debug, Default)]
pub struct App {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub app_type: Option<u32>,
    pub category: String,
    pub version: String,
    pub path: PathBuf,
    pub sfo: SfoDataTable,
}

#[derive(Clone, Debug)]
pub struct Patch {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub app_type: Option<u32>,
    pub category: String,
    pub version: String,
    pub path: PathBuf,
    pub sfo: SfoDataTable,
}

pub fn scan(path: &Path) -> Vec<Title> {
    let mut list: Vec<Title> = Vec::new();
    let mut patches: HashMap<String, Patch> = HashMap::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let game_dir = entry.path();
                        if !game_dir.is_dir() {
                            continue;
                        }

                        let sfo_path = game_dir.join("sce_sys").join("param.sfo");
                        if !sfo_path.is_file() {
                            continue;
                        }

                        let data_table = match read_sfo(&sfo_path) {
                            Ok(sfo) => sfo.data_table,
                            Err(error) => {
                                eprintln!("Skipping {}: {error}", sfo_path.display());
                                continue;
                            }
                        };
                        let parent = game_dir.parent().unwrap_or(path).to_path_buf();
                        let serial = data_table.find_string("TITLE_ID").unwrap(); // serial
                        let name = data_table.find_string("TITLE").unwrap();
                        let system_ver = data_table.find_integer("SYSTEM_VER").unwrap();
                        let fw = format!(
                            "{:x}.{:02x}",
                            (system_ver >> 24) & 0xff,
                            (system_ver >> 16) & 0xff
                        );
                        let version = data_table.find_string("APP_VER").unwrap();
                        let category = data_table.find_string("CATEGORY").unwrap();
                        let app_type = data_table.find_integer("APP_TYPE");
                        let folder_name =
                            game_dir.file_name().unwrap().to_string_lossy().into_owned();
                        let is_patch =
                            folder_name.ends_with("-patch") || folder_name.ends_with("-UPDATE");
                        let icon = find_icon(&game_dir);
                        let background = find_background(&game_dir);
                        let attribute_bytes = data_table
                            .find_integer("ATTRIBUTE")
                            .unwrap_or_default()
                            .to_le_bytes();

                        if is_patch {
                            let patch = Patch {
                                name,
                                fw,
                                category,
                                app_type,
                                version,
                                path: game_dir,
                                sfo: data_table,
                            };

                            // shadPS4 prefers -UPDATE when both update folder names exist.
                            let prefer_update = folder_name.ends_with("-UPDATE");
                            if prefer_update || !patches.contains_key(&serial) {
                                patches.insert(serial, patch);
                            }
                        } else {
                            list.push(Title {
                                icon_path: icon,
                                background_path: background,
                                parent_dir: parent,
                                serial,
                                compatibility: None,
                                publisher_id: data_table.find_string("CONTENT_ID").unwrap(),
                                playtime: String::new(),
                                supports_vr: supports_vr(attribute_bytes),
                                supports_neo: supports_neo(attribute_bytes),
                                app: App {
                                    name,
                                    fw,
                                    category,
                                    app_type,
                                    version,
                                    path: game_dir,
                                    sfo: data_table,
                                },
                                patch: None,
                            });
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    for title in &mut list {
        title.patch = patches.remove(&title.serial);
        let attribute_bytes = title
            .patch
            .as_ref()
            .and_then(|patch| patch.sfo.find_integer("ATTRIBUTE"))
            .or_else(|| title.app.sfo.find_integer("ATTRIBUTE"))
            .unwrap_or_default()
            .to_le_bytes();
        title.supports_vr = attribute::supports_vr(attribute_bytes);
        title.supports_neo = attribute::supports_neo(attribute_bytes);
        if let Some(patch) = &title.patch {
            if let Some(icon) = find_icon(&patch.path) {
                title.icon_path = Some(icon);
            }
            if let Some(background) = find_background(&patch.path) {
                title.background_path = Some(background);
            }
        }
    }
    list
}

fn existing_file(path: PathBuf) -> Option<PathBuf> {
    path.is_file().then_some(path)
}

fn find_icon(game_dir: &Path) -> Option<PathBuf> {
    ["icon0.png", "icon1.png"]
        .into_iter()
        .find_map(|name| existing_file(game_dir.join("sce_sys").join(name)))
}

fn find_background(game_dir: &Path) -> Option<PathBuf> {
    ["pic0.png", "pic1.png"]
        .into_iter()
        .find_map(|name| existing_file(game_dir.join("sce_sys").join(name)))
}
