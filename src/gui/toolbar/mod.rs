use eframe::egui;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};

#[derive(Default)]
pub struct Toolbar {
    games_path: Option<PathBuf>,
    folder_rx: Option<Receiver<Option<PathBuf>>>,
}

impl Toolbar {
    pub fn games_path(&self) -> Option<&Path> {
        self.games_path.as_deref()
    }

    pub fn can_refresh(&self) -> bool {
        self.games_path.is_some() && self.folder_rx.is_none()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, scanning: bool) -> bool {
        let mut changed = false;
        if let Some(receiver) = &self.folder_rx {
            match receiver.try_recv() {
                Ok(path) => {
                    if let Some(path) = path {
                        self.games_path = Some(path);
                        changed = true;
                    }
                    self.folder_rx = None;
                }
                Err(TryRecvError::Disconnected) => self.folder_rx = None,
                Err(TryRecvError::Empty) => {}
            }
        }

        egui::Panel::top("games_folder")
            .resizable(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(
                            !scanning && !changed && self.folder_rx.is_none(),
                            egui::Button::new("Browse…"),
                        )
                        .clicked()
                    {
                        self.browse(ui.ctx());
                    }
                    if self.folder_rx.is_some() {
                        ui.spinner();
                    }
                    if let Some(path) = &self.games_path {
                        let path = path.to_string_lossy();
                        ui.add(egui::Label::new(path.as_ref()).truncate())
                            .on_hover_text(path.as_ref());
                    } else {
                        ui.label("No folder selected");
                    }
                });
            });
        changed
    }

    fn browse(&mut self, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        self.folder_rx = Some(rx);
        let current_path = self.games_path.clone();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new().set_title("Select games folder");
            if let Some(path) = current_path {
                dialog = dialog.set_directory(path);
            }
            let _ = tx.send(dialog.pick_folder());
            ctx.request_repaint();
        });
    }
}
