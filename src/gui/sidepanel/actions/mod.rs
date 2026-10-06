pub mod search;

pub struct SidepanelActions<'a> {
    pub search: Option<&'a str>,
    pub refresh_list: bool,
}
