use crate::sfo::sfo::read_sfo;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Title {
    pub parent_dir: PathBuf,
    pub serial: String,
    pub compatibility: Option<String>,
    pub publisher_id: String,
    pub playtime: String,
    pub app: App,
    pub patch: Option<Patch>, // Maybe a title doesn't have a patch
}

#[derive(Debug)]
pub struct App {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub size: u64,
    pub version: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Patch {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub size: u64,
    pub version: String,
    pub path: PathBuf,
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

                        let data_table = read_sfo(&sfo_path).unwrap().data_table;
                        let size_bytes = match directory_size(&game_dir) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                eprintln!("Could not measure {}: {err}", game_dir.display());
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
                        let folder_name =
                            game_dir.file_name().unwrap().to_string_lossy().into_owned();
                        let is_patch =
                            folder_name.ends_with("-patch") || folder_name.ends_with("-UPDATE");

                        if is_patch {
                            let patch = Patch {
                                name,
                                fw,
                                size: size_bytes,
                                version,
                                path: game_dir,
                            };

                            // shadPS4 prefers -UPDATE when both update folder names exist.
                            let prefer_update = folder_name.ends_with("-UPDATE");
                            if prefer_update || !patches.contains_key(&serial) {
                                patches.insert(serial, patch);
                            }
                        } else {
                            list.push(Title {
                                parent_dir: parent,
                                serial,
                                compatibility: None,
                                publisher_id: data_table.find_string("CONTENT_ID").unwrap(),
                                playtime: String::new(),
                                app: App {
                                    name,
                                    fw,
                                    size: size_bytes,
                                    version,
                                    path: game_dir,
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
    }
    list
}

fn directory_size(path: &Path) -> std::io::Result<u64> {
    let mut bytes = 0;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;

        if kind.is_dir() {
            bytes += directory_size(&entry.path())?;
        } else if kind.is_file() {
            bytes += entry.metadata()?.len();
        }
    }

    Ok(bytes)
}
