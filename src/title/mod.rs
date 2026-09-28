pub mod app;
mod app_union;
pub mod detect;
pub mod dlc;
pub mod patch;
pub mod sfo;

use crate::title::app_union::AppUnion;
pub use app::App;
pub use dlc::DLC;
pub use patch::Patch;

#[derive(Debug)]
pub struct Title {
    pub title_id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub app: App,
    pub patch: Option<Patch>,
    pub app_union: Option<AppUnion>,
    pub dlcs: Vec<DLC>,
}
