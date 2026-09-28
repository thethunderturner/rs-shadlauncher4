use crate::title::detect::PFSMNT_PATH;

#[derive(Debug)]
pub struct Patch {
    pub path: String,
}

pub fn find_patch(mounts: &[String], current: &str) -> Option<Patch> {
    let target = format!("{current}-patch0");

    match mounts.iter().find(|mount| mount.as_str() == target) {
        Some(mount) => Some(Patch {
            path: format!("{PFSMNT_PATH}/{mount}"),
        }),

        None => None,
    }
}
