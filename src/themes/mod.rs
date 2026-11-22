pub mod dark;
pub mod light;

use crate::themes::dark::dark;
use crate::themes::light::light;
use eframe::egui::Visuals;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AppTheme {
    Light = 0,
    Dark = 1,
}

impl AppTheme {
    pub const ALL: &'static [AppTheme] = &[AppTheme::Light, AppTheme::Dark];

    pub fn from_id(id: u32) -> Option<AppTheme> {
        Self::ALL
            .iter()
            .copied()
            .find(|app_theme| *app_theme as u32 == id)
    }

    pub fn label(app_theme: AppTheme) -> String {
        String::from(match app_theme {
            AppTheme::Dark => "Dark",
            AppTheme::Light => "Light",
        })
    }

    pub fn visuals(&self) -> Visuals {
        match self {
            Self::Light => light(),
            Self::Dark => dark(),
        }
    }
}
