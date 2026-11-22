use crate::app::State;
use crate::library::scan::Title;
use eframe::egui;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Filter {
    #[default]
    All,
    Demos,
    International,
    TV,
    VR,
    Neo,
}

impl Filter {
    pub fn label(self) -> String {
        match self {
            Filter::All => String::from("All"),
            Filter::Demos => String::from("Demos"),
            Filter::International => String::from("Default Apps"),
            Filter::TV => String::from("TV Apps"),
            Filter::VR => String::from("VR Support"),
            Filter::Neo => String::from("Pro Support"),
        }
    }

    pub fn matches(self, title: &Title) -> bool {
        match self {
            Self::All => true,
            Self::Demos => title.app_type == Some(3),
            Self::International => title.publisher_id.starts_with("IP"),
            Self::TV => title.category == "gde" || title.category == "gpe",
            Self::VR => title.supports_vr,
            Self::Neo => title.supports_neo,
        }
    }
}

#[derive(Default)]
pub struct Sidebar {
    pub search: String,
    pub filter: Filter,
    pub indices: Vec<usize>,
}

impl Sidebar {
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        egui::Panel::left("sidebar")
            .default_size(175.0)
            .max_size(250.0)
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("Search");
                if ui
                    .add(
                        egui::TextEdit::singleline(&mut self.search)
                            .id_salt("game_search")
                            .hint_text("Title or CUSA")
                            .desired_width(f32::INFINITY),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.separator();
                ui.heading("Filters");
                for filter in [
                    Filter::All,
                    Filter::Demos,
                    Filter::International,
                    Filter::TV,
                    Filter::VR,
                    Filter::Neo,
                ] {
                    if ui
                        .add_sized(
                            [ui.available_width(), 28.0],
                            egui::Button::new(filter.label()).selected(self.filter == filter),
                        )
                        .clicked()
                        && self.filter != filter
                    {
                        self.filter = filter;
                        changed = true;
                    }
                }
                ui.separator();
                ui.add_enabled_ui(false, |ui| {
                    ui.add_sized([ui.available_width(), 28.0], egui::Button::new("New +"))
                        .on_hover_text("Feature not available yet");
                });
            });

        changed
    }

    pub fn search(&self, state: &State) -> Vec<usize> {
        let query = self.search.trim().to_lowercase();
        let visible_indices: Vec<usize> = state
            .loaded_titles
            .loaded_titles
            .iter()
            .enumerate()
            .filter(|(_, title)| {
                self.filter.matches(&title.title)
                    && (query.is_empty()
                        || title.title.name.to_lowercase().contains(&query)
                        || title.title.serial.to_lowercase().contains(&query))
            })
            .map(|(index, _)| index)
            .collect();
        visible_indices
    }
}
