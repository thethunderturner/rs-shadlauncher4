use crate::scanning::Title;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LibraryFilter {
    #[default]
    All,
    Demos,
    International,
    TV,
    VR,
    Neo,
}

impl LibraryFilter {
    pub fn matches(self, title: &Title) -> bool {
        match self {
            Self::All => true,
            Self::Demos => title.app.app_type == Some(3),
            Self::International => title.publisher_id.starts_with("IP"),
            Self::TV => title.app.category == "gde",
            Self::VR => title.supports_vr,
            Self::Neo => title.supports_neo,
        }
    }
}

#[derive(Default)]
pub struct Library {
    pub selected: LibraryFilter,
}
