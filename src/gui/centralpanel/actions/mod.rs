use crate::gui::centralpanel::actions::row::Row;

pub mod assets;
pub mod row;
pub mod scan;

#[derive(Default)]
pub struct Actions {
    pub row: Row,
}
