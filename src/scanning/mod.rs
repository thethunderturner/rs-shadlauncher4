use crate::sfo::sfo::read_sfo;
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
    pub patch: Option<Patch> // Maybe a title doesn't have a patch
}

#[derive(Debug)]
struct App {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub size: u64,
    pub version: String,
    pub path: PathBuf,
}

#[derive(Debug)]
struct Patch {
    pub name: String, // Keeping the name here, because some titles change names between versions
    pub fw: String,
    pub size: u64,
    pub version: String,
    pub path: PathBuf,
}

pub fn scan(path: &Path) -> Vec<Title> {
    let mut list: Vec<Title> = Vec::new();
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

                        let parent = game_dir.parent();


                        // list.push(Title {
                        //     name: data_table.find_string("TITLE").unwrap(),
                        //     serial: data_table.find_string("TITLE_ID").unwrap(),
                        //     compatibility: None,
                        //     publisher_id: data_table.find_string("CONTENT_ID").unwrap(),
                        //     fw: data_table.find_integer("SYSTEM_VER").unwrap().to_string(),
                        //     size: format!("{:.2} GiB", size_bytes as f64 / 1024_f64.powi(3)),
                        //     version: data_table.find_string("VERSION").unwrap(),
                        //     playtime: String::new(),
                        //     path: game_dir,
                        // });
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
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
