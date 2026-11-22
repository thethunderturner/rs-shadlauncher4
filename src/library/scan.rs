use crate::config::emulator::Config;
use crate::library::cache::Db;
use crate::library::title::{Title, find_background, find_icon};
use crate::sfo::parse_param::attribute::{supports_neo, supports_vr};
use crate::sfo::reader::read_sfo;
use std::collections::BTreeMap;
use std::fs;

pub struct Scan;

impl Scan {
    pub fn scan(mut progress: impl FnMut(Option<f32>, String)) -> Vec<Title> {
        let config = Config::load();
        let mut titles: BTreeMap<String, Title> = BTreeMap::new();

        progress(None, "Finding game folders".into());
        let mut folders = Vec::new();
        for directory in &config.general.install_dirs {
            if !directory.enabled {
                continue;
            }

            let entries = match fs::read_dir(&directory.path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Failed to read {:?}: {e}", directory.path);
                    continue;
                }
            };

            for entry in entries.flatten() {
                folders.push((directory.path.clone(), entry.path()));
            }
        }
        let total = folders.len();
        for (index, (parent_dir, title_path)) in folders.into_iter().enumerate() {
            progress(
                Some(index as f32 / total.max(1) as f32),
                format!("Scanning folder {} of {total}", index + 1),
            );
            let sfo_path = title_path.join("sce_sys").join("param.sfo");

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

            let Some(serial) = data_table.find_string("TITLE_ID") else {
                eprintln!("Skipping {}: no TITLE_ID", sfo_path.display());
                continue;
            };

            let folder_name = title_path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let is_patch = folder_name.ends_with("-patch") || folder_name.ends_with("-UPDATE");

            let attribute_bytes = data_table
                .find_integer("ATTRIBUTE")
                .unwrap_or_default()
                .to_le_bytes();

            let title = titles.entry(serial.clone()).or_insert_with(|| Title {
                parent_dir: parent_dir.clone(),
                serial,
                ..Default::default()
            });

            // Patch metadata wins, even when the app is scanned afterward.
            if is_patch || title.patch_path.is_none() {
                title.name = data_table.find_string("TITLE").unwrap_or_default();
                title.publisher_id = data_table.find_string("CONTENT_ID").unwrap_or_default();
                title.version = data_table.find_string("APP_VER").unwrap_or_default();
                title.fw = data_table.find_integer("SYSTEM_VER").unwrap_or_default();
                title.app_type = data_table.find_integer("APP_TYPE");
                title.category = data_table.find_string("CATEGORY").unwrap_or_default();
                title.supports_vr = supports_vr(attribute_bytes);
                title.supports_neo = supports_neo(attribute_bytes);
            }

            if is_patch {
                title.patch_path = Some(title_path);
            } else {
                title.app_path = Some(title_path);
                title.parent_dir = parent_dir;
            }
        }

        progress(None, "Finding title artwork".into());
        // Choose artwork after scanning so folder order cannot change the result.
        for title in titles.values_mut() {
            title.icon_path = title
                .patch_path
                .as_deref()
                .and_then(find_icon)
                .or_else(|| title.app_path.as_deref().and_then(find_icon));
            title.background_path = title
                .patch_path
                .as_deref()
                .and_then(find_background)
                .or_else(|| title.app_path.as_deref().and_then(find_background));
        }
        let titles: Vec<Title> = titles.into_values().collect();

        let mut db = Db::connect().expect("Failed to open cache");
        db.create_tables().expect("Failed to create cache tables");
        db.save_titles_with_progress(&titles, |done, total| {
            progress(
                Some(done as f32 / total.max(1) as f32),
                format!("Saving titles and artwork: {done} of {total}"),
            );
        })
        .expect("Failed to save titles");
        progress(None, "Loading the updated table".into());

        titles
    }
}
