pub mod library;
pub mod refresh;
pub mod search;

pub struct SidepanelActions<'a> {
    pub search: Option<&'a str>,
    pub library_filter: Option<library::LibraryFilter>,
    pub refresh_list: bool,
    pub refresh_compatibility: bool,
}
