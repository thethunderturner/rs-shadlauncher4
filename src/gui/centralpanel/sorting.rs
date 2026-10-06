use crate::scanning::Title;
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
                    egui::include_image!("../../assets/centralpanel/columns/arrow-up.svg")
                }
                Order::Descending => {
                    egui::include_image!("../../assets/centralpanel/columns/arrow-down.svg")
                }
            };
            let image = egui::Image::new(arrow).fit_to_exact_size(egui::vec2(12.0, 12.0));
            egui::Button::new((text, image)).image_tint_follows_text_color(true)
        } else {
            egui::Button::new(text)
        };
        button.frame(false)
    }

    pub fn sort(&self, titles: &mut [Title]) {
        titles.sort_by(|a, b| {
            let comparison = match self.column.as_str() {
                // Icons are currently placeholders; group by available icon files.
                "icon" => a.icon_path.is_some().cmp(&b.icon_path.is_some()),
                "name" => {
                    let a = a.patch.as_ref().map_or(&a.app.name, |patch| &patch.name);
                    let b = b.patch.as_ref().map_or(&b.app.name, |patch| &patch.name);
                    a.to_lowercase().cmp(&b.to_lowercase())
                }
                "serial" => a.serial.cmp(&b.serial),
                "firmware" => {
                    let a = a.patch.as_ref().map_or(&a.app.fw, |patch| &patch.fw);
                    let b = b.patch.as_ref().map_or(&b.app.fw, |patch| &patch.fw);
                    compare_versions(a, b)
                }
                "version" => {
                    let a = a
                        .patch
                        .as_ref()
                        .map_or(&a.app.version, |patch| &patch.version);
                    let b = b
                        .patch
                        .as_ref()
                        .map_or(&b.app.version, |patch| &patch.version);
                    compare_versions(a, b)
                }
                "path" => a.parent_dir.cmp(&b.parent_dir),
                _ => Ordering::Equal,
            };
            match self.order {
                Order::Ascending => comparison,
                Order::Descending => comparison.reverse(),
            }
        });
    }
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
