use eframe::egui;

pub struct View {}

impl View {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            let _ = ui.button("Show Tile Bars");
            let _ = ui.button("Show Tool Bar");
            let _ = ui.button("Show Title List");
            let _ = ui.button("Show Hidden Entries");
            let _ = ui.button("Show Game Compatibility in Grid Mode");
            let _ = ui.button("Game List Refresh");
            ui.menu_button("Game List Mode", |ui| {
                let _ = ui.button("List");
                let _ = ui.button("Grid");
            });
            ui.menu_button("Game List Icons", |ui| {
                let _ = ui.button("Tiny");
                let _ = ui.button("Small");
                let _ = ui.button("Medium");
                let _ = ui.button("Large");
            });
            let _ = ui.button("Show Game Log");
        });
    }
}
