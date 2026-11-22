use crate::app::State;
use eframe::egui;

pub mod dialogs;
pub mod footer;
pub mod menubar;
pub mod settings;
pub mod sidebar;
pub mod table;
pub mod toolbar;
pub mod widgets;
pub mod windows;

#[derive(Default)]
pub struct Gui {
    menubar: menubar::MenuBar,
    toolbar: toolbar::Toolbar,
    sidebar: sidebar::Sidebar,
    table: table::Table,
    footer: footer::Footer,
}

impl Gui {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut State) {
        self.menubar.show(ui, state);
        if self.sidebar.show(ui) {
            state.visible_indices = self.sidebar.search(state);
        }
        self.toolbar.show(ui, state);
        self.footer.show(ui, state);
        self.table.show(ui, state);
    }
}
