pub mod config;
pub mod keys;
pub mod users;

use serde::Serialize;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

const USER_SUBDIRECTORIES: &[&str] = &[
    "versions",
    "addcont",
    "home",
    "sys_modules",
    "log",
    "custom_configs",
    "custom_input_configs",
    "custom_trophy",
    "patches",
    "cheats",
    "cache",
    "fonts",
    "themes",
];

// Create the writable user-data directory beside the executable.
pub fn initialize_user_directory() -> io::Result<PathBuf> {
    let user_dir = user_directory()?;

    if !user_dir.exists() {
        std::fs::create_dir_all(&user_dir)?;
    }
    for directory in USER_SUBDIRECTORIES {
        std::fs::create_dir_all(user_dir.join(directory))?;
    }

    create_default_json::<config::Config>(&user_dir.join("config.json"))?;
    create_default_json::<keys::Keys>(&user_dir.join("keys.json"))?;
    create_default_json::<users::Users>(&user_dir.join("users.json"))?;

    Ok(user_dir)
}

pub fn user_directory() -> io::Result<PathBuf> {
    Ok(executable_directory()?.join("user"))
}

fn create_default_json<T: Default + Serialize>(path: &Path) -> io::Result<()> {
    let mut contents = serde_json::to_vec_pretty(&T::default()).map_err(io::Error::other)?;
    contents.push(b'\n');

    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => file.write_all(&contents),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error),
    }
}

fn executable_directory() -> io::Result<PathBuf> {
    // AppImage runs its binary from a temporary, read-only mount. APPIMAGE
    // points to the actual executable file, so keep portable data beside it.
    if let Some(appimage) = std::env::var_os("APPIMAGE") {
        let path = PathBuf::from(appimage);
        if let Some(parent) = path.parent() {
            return Ok(parent.to_owned());
        }
    }

    std::env::current_exe()?
        .parent()
        .map(Path::to_owned)
        .ok_or_else(|| io::Error::other("executable path has no parent directory"))
}
