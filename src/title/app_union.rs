use crate::title::detect::PFSMNT_PATH;

#[derive(Debug)]
pub struct AppUnion {
    pub path: String,
}

pub fn find_app_union(mounts: &[String], current: &str) -> Option<AppUnion> {
    let target = format!("{current}-app0-patch0-union");

    match mounts.iter().find(|mount| mount.as_str() == target) {
        Some(mount) => Some(AppUnion {
            path: format!("{PFSMNT_PATH}/{mount}"),
        }),

        None => None,
    }
}
