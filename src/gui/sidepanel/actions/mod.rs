use crate::gui::sidepanel::actions::search::Search;

pub mod search;

pub struct SidepanelActions<'a> {
    pub search: Search<'a>
}