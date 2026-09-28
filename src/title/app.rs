use crate::title::detect::PFSMNT_PATH;
use std::collections::HashSet;

#[derive(Debug)]
pub struct App {
    pub path: String,
}

pub fn find_app(mounts: &[String], current: &str) -> Result<App, String> {
    let target = format!("{current}-app0");

    match mounts.iter().find(|mount| mount.as_str() == target) {
        Some(mount) => Ok(App {
            path: format!("{}/{}", PFSMNT_PATH, mount.clone()),
        }),
        None => Err(format!("Could not find app mount for {current}")),
    }
}

pub fn find_title_id(mounts: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut title_ids = Vec::new();

    for mount in mounts {
        if let Some((title_id, _)) = mount.split_once('-') {
            if seen.insert(title_id.to_string()) {
                title_ids.push(title_id.to_string());
            }
        }
    }

    title_ids
}
