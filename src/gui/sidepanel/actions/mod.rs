pub mod search;
pub mod library;

pub struct SidepanelActions<'a> {
    pub search: Option<&'a str>,
    pub refresh_list: bool,
}
