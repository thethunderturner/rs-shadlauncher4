use crate::config::emulator::Config;
use crate::gui::windows::progress;
use crate::gui::windows::progress::Progress;
use crate::library::cache::{Db, LoadedTitles};
use crate::library::title::{Title, find_background, find_icon};
use crate::sfo::parse_param::attribute::{supports_neo, supports_vr};
use crate::sfo::reader::read_sfo;
use std::collections::BTreeMap;
use std::fs;

#[derive(Default)]
pub struct Scan {
    receiver: Option<std::sync::mpsc::Receiver<ScanEvent>>,
    progress: Option<Progress>,
    error: Option<String>,
}

enum ScanEvent {
    Progress(Progress),
    Finished(Result<LoadedTitles, String>),
}

impl Scan {
    pub fn start(&mut self, ctx: &eframe::egui::Context, config: &Config) {
        if self.receiver.is_some() {
            return;
        }

        self.error = None;
        self.progress = Some(Progress {
            title: "Scanning titles".into(),
            fraction: None,
            message: "Starting scan".into(),
        });
        let (sender, receiver) = std::sync::mpsc::channel();
        let ctx = ctx.clone();
        let config = config.clone();
        let worker = std::thread::Builder::new()
            .name("title-scan".into())
            .spawn(move || {
                let result = Self::scan(&config, |fraction, message| {
                    let _ = sender.send(ScanEvent::Progress(Progress {
                        title: "Scanning titles".into(),
                        fraction,
                        message,
                    }));
                    ctx.request_repaint();
                })
                .and_then(|_| LoadedTitles::try_load().map_err(|error| error.to_string()));
                let _ = sender.send(ScanEvent::Finished(result));
                ctx.request_repaint();
            });

        match worker {
            Ok(_) => self.receiver = Some(receiver),
            Err(error) => {
                self.progress = None;
                self.error = Some(format!("Could not start scan: {error}"));
            }
        }
    }

    pub fn show(&mut self, ui: &mut eframe::egui::Ui) -> Option<LoadedTitles> {
        let mut completed = None;
        while let Some(receiver) = &self.receiver {
            match receiver.try_recv() {
                Ok(ScanEvent::Progress(progress)) => self.progress = Some(progress),
                Ok(ScanEvent::Finished(result)) => {
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
                    self.error =
                        Some("The scan worker stopped before completing. Try scanning again.".into());
                    break;
                }
            }
        }

        if let Some(progress) = &self.progress {
            progress::show(ui.ctx(), progress);
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }
        if let Some(error) = &self.error {
            let mut open = true;
            eframe::egui::Window::new("Scan failed")
                .id(eframe::egui::Id::unique("scan_error"))
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

    pub fn scan(
        config: &Config,
        mut progress: impl FnMut(Option<f32>, String),
    ) -> Result<Vec<Title>, String> {
        if !config
            .general
            .install_dirs
            .iter()
            .any(|directory| directory.enabled)
        {
            return Err("No game folders found or enabled. Add or enable a folder in Settings -> Paths, then Save or Apply before refreshing.".into());
        }
        let mut titles: BTreeMap<String, Title> = BTreeMap::new();

        progress(None, "Finding game folders".into());
        let mut folders = Vec::new();
        for directory in &config.general.install_dirs {
            if !directory.enabled {
                continue;
            }

            if directory.path.as_os_str().is_empty() {
                return Err(
                    "An enabled game folder has no path. Choose a folder in Settings -> Paths.".into()
                );
            }
            let entries = fs::read_dir(&directory.path).map_err(|error| {
                format!("Could not read game folder {}: {error}", directory.path.display())
            })?;

            for entry in entries {
                let entry = entry.map_err(|error| {
                    format!("Could not read an entry in {}: {error}", directory.path.display())
                })?;
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

        if titles.is_empty() {
            return Err("No valid titles found. In Settings → Paths, select the folder containing your extracted game folders. Each game must contain sce_sys/param.sfo. The existing cache was kept.".into());
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

        let mut db = Db::connect().map_err(|error| format!("Failed to open cache: {error}"))?;
        db.create_tables()
            .map_err(|error| format!("Failed to create cache tables: {error}"))?;
        db.save_titles_with_progress(&titles, |done, total| {
            progress(
                Some(done as f32 / total.max(1) as f32),
                format!("Caching titles and artwork: {done} of {total}"),
            );
        })
        .map_err(|error| format!("Failed to save titles: {error}"))?;
        progress(None, "Loading the updated table".into());

        Ok(titles)
    }
}
