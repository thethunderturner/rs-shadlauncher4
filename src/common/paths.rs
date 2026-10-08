use std::io;
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
    let base_dir = executable_directory()?;
    let user_dir = base_dir.join("user");

    std::fs::create_dir_all(&user_dir)?;
    for directory in USER_SUBDIRECTORIES {
        std::fs::create_dir_all(user_dir.join(directory))?;
    }

    Ok(user_dir)
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
