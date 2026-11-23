use crate::library::cache::LoadedTitle;
use eframe::egui;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Ascending,
    Descending,
}

pub struct Sorting {
    pub column: String,
    pub order: Order,
}
impl Default for Sorting {
    fn default() -> Self {
        Self {
            column: String::from("name"),
            order: Order::Ascending,
        }
    }
}

impl Sorting {
    pub fn select_column(&mut self, column: &str) {
        self.order = if self.column == column && self.order == Order::Ascending {
            Order::Descending
        } else {
            Order::Ascending
        };
        self.column = column.to_owned();
    }

    pub fn header(&self, column: &str, label: &str) -> egui::Button<'static> {
        let text = egui::RichText::new(label).strong();
        let button = if self.column == column {
            let arrow = match self.order {
                Order::Ascending => {
                    egui::include_image!("../assets/table/columns/arrow-up.svg")
                }
                Order::Descending => {
                    egui::include_image!("../assets/table/columns/arrow-down.svg")
                }
            };
            let image = egui::Image::new(arrow).fit_to_exact_size(egui::vec2(12.0, 12.0));
            egui::Button::new((text, image)).image_tint_follows_text_color(true)
        } else {
            egui::Button::new(text)
        };
        button.frame(false)
    }

    pub fn sort(&self, titles: &[LoadedTitle], indices: &mut [usize]) {
        indices.sort_by(|&a, &b| {
            let a = &titles[a];
            let b = &titles[b];
            let comparison = match self.column.as_str() {
                "icon" => a.icon.is_some().cmp(&b.icon.is_some()),
                "name" => a.title.name.to_lowercase().cmp(&b.title.name.to_lowercase()),
                "status" | "compatibility" | "compatibility_status" => {
                    status_rank(a.compatibility.as_deref()).cmp(&status_rank(b.compatibility.as_deref()))
                }
                "serial" => a.title.serial.cmp(&b.title.serial),
                "region" => region_from_content_id(&a.title.publisher_id)
                    .cmp(region_from_content_id(&b.title.publisher_id)),
                "firmware" => a.title.fw.cmp(&b.title.fw),
                "version" => compare_versions(&a.title.version, &b.title.version),
                "path" => {
                    let a = a.title.app_path.as_ref().or(a.title.patch_path.as_ref());
                    let b = b.title.app_path.as_ref().or(b.title.patch_path.as_ref());
                    a.map(|p| p.to_string_lossy().to_lowercase())
                        .unwrap_or_default()
                        .cmp(&b.map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default())
                }
                "milestone" => a.title.milestone.cmp(&b.title.milestone),
                _ => Ordering::Equal,
            };
            match self.order {
                Order::Ascending => comparison,
                Order::Descending => comparison.reverse(),
            }
        });
    }
}

fn status_rank(status: Option<&str>) -> usize {
    ["Playable", "Ingame", "Menus", "Boots", "Nothing"]
        .iter()
        .position(|known| status.is_some_and(|status| status.eq_ignore_ascii_case(known)))
        .unwrap_or(5)
}

// Compare numeric components so 10.00 sorts after 9.00.
fn compare_versions(a: &str, b: &str) -> Ordering {
    let mut a = a.split('.');
    let mut b = b.split('.');
    loop {
        let comparison = match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(a), Some(b)) => match (a.parse::<u64>(), b.parse::<u64>()) {
                (Ok(a), Ok(b)) => a.cmp(&b),
                _ => a.cmp(b),
            },
        };
        if comparison != Ordering::Equal {
            return comparison;
        }
    }
}

fn region_from_content_id(content_id: &str) -> &'static str {
    match content_id.get(..2) {
        Some("UP" | "UB" | "UT" | "US") => "USA",
        Some("EP" | "EB") => "Europe",
        Some("JP" | "JB") => "Japan",
        Some("HP" | "HB") => "Asia",
        Some("IP") => "World",
        _ => "Unknown",
    }
}
