use super::Title;
use crate::compatibility::{self, Database};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct Snapshot {
    version: u32,
    roots: Vec<PathBuf>,
    titles: Vec<Title>,
}

pub fn load(dir: &Path, roots: &[PathBuf], database: &Database) -> Option<Vec<Title>> {
    let bytes = fs::read(dir.join("library.json")).ok()?;
    let mut snapshot: Snapshot = serde_json::from_slice(&bytes).ok()?;
    if snapshot.version != VERSION || snapshot.roots != roots {
        return None;
    }
    for title in &mut snapshot.titles {
        for path in [&title.icon_path, &title.background_path].into_iter().flatten() {
            if path.starts_with(dir) && !path.is_file() {
                return None;
            }
        }
        // Compatibility can change independently of the installed games.
        title.compatibility = database.compatibility_for(&title.serial, compatibility::current_os_key());
    }
    Some(snapshot.titles)
}

pub fn save(
    dir: &Path,
    roots: &[PathBuf],
    titles: &mut [Title],
    mut on_progress: impl FnMut(),
) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    // Publish metadata only after its artwork is complete. Interrupted refreshes
    // leave the previous snapshot and its images intact.
    let previous: Option<Snapshot> = fs::read(dir.join("library.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok());
    let artwork = tempfile::Builder::new().prefix("artwork-").tempdir_in(dir)?;
    let mut snapshot = Snapshot {
        version: VERSION,
        roots: roots.to_vec(),
        titles: titles.to_vec(),
    };
    for (index, title) in snapshot.titles.iter_mut().enumerate() {
        cache_image(&mut title.icon_path, artwork.path(), &format!("{index}-icon.png"), (80, 80));
        cache_image(
            &mut title.background_path,
            artwork.path(),
            &format!("{index}-background.png"),
            (1280, 720),
        );
        on_progress();
    }
    let mut file = tempfile::NamedTempFile::new_in(dir)?;
    serde_json::to_writer(file.as_file_mut(), &snapshot)?;
    file.flush()?;
    file.as_file().sync_all()?;
    file.persist(dir.join("library.json"))
        .map_err(|error| error.error)?;
    let _ = artwork.keep();
    titles.clone_from_slice(&snapshot.titles);
    // Remove only the old snapshot's generated artwork directories.
    if let Some(previous) = previous {
        let directories: std::collections::HashSet<_> = previous
            .titles
            .iter()
            .flat_map(|title| [&title.icon_path, &title.background_path])
            .filter_map(|path| path.as_ref()?.parent())
            .filter(|path| {
                path.parent() == Some(dir)
                    && path
                        .file_name()
                        .is_some_and(|name| name.to_string_lossy().starts_with("artwork-"))
            })
            .collect();
        for directory in directories {
            if let Err(error) = fs::remove_dir_all(directory) {
                eprintln!("Could not remove old cached artwork: {error}");
            }
        }
    }
    Ok(())
}

fn cache_image(source: &mut Option<PathBuf>, dir: &Path, name: &str, size: (u32, u32)) {
    let Some(path) = source.as_ref() else { return };
    let result = (|| -> Result<PathBuf, Box<dyn std::error::Error>> {
        let image = image::ImageReader::open(path)?.decode()?;
        let thumbnail = image.thumbnail(size.0, size.1);
        let mut file = tempfile::NamedTempFile::new_in(dir)?;
        thumbnail.write_to(file.as_file_mut(), image::ImageFormat::Png)?;
        let destination = dir.join(name);
        file.persist(&destination)?;
        Ok(destination)
    })();
    match result {
        Ok(path) => *source = Some(path),
        // An unavailable/bad image or unwritable cache must not hide a game.
        Err(error) => eprintln!("Could not cache artwork {}: {error}", path.display()),
    }
}

pub fn invalidate() {
    if let Ok(user) = crate::common::user_directory() {
        let path = user.join("cache/library/library.json");
        if let Err(error) = fs::remove_file(path) {
            if error.kind() != io::ErrorKind::NotFound {
                eprintln!("Could not invalidate game cache: {error}");
            }
        }
    }
}
