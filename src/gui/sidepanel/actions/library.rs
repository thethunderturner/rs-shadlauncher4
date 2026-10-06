use crate::scanning::Title;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LibraryFilter {
    #[default]
    All,
    Demos,
    International,
    TV
}

impl LibraryFilter {
    pub fn matches(self, title: &Title) -> bool {
        match self {
            Self::All => true,
            Self::Demos => title.app.app_type == Some(3),
            Self::International => title.publisher_id.starts_with("IP"),
            Self::TV => title.app.category == "gde"
        }
    }
}

#[derive(Default)]
pub struct Library {
    pub selected: LibraryFilter,
}
