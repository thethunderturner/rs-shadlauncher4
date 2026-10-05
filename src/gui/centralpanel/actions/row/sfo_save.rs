use crate::sfo::data_table::SfoDataTable;
use crate::sfo::writer::write_sfo;
use eframe::egui;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, TryRecvError};

#[derive(Default)]
pub(super) struct SfoSave {
    original: Option<SfoDataTable>,
    receiver: Option<Receiver<Result<SfoDataTable, String>>>,
    result: Option<Result<(), String>>,
}

impl SfoSave {
    pub(super) fn is_saving(&self) -> bool {
        self.receiver.is_some()
    }

    pub(super) fn poll(&mut self) -> Option<SfoDataTable> {
        let receiver = self.receiver.as_ref()?;
        let result = match receiver.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return None,
            Err(TryRecvError::Disconnected) => Err("The SFO save stopped before finishing.".into()),
        };
        self.receiver = None;
        match result {
            Ok(table) => {
                self.original = Some(table.clone());
                self.result = Some(Ok(()));
                Some(table)
            }
            Err(error) => {
                self.result = Some(Err(error));
                None
            }
        }
    }

    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
        game_path: &Path,
        table: &SfoDataTable,
        editing: bool,
    ) {
        let original = self.original.get_or_insert_with(|| table.clone());
        let changed = original != table;
        let path = game_path.join("sce_sys/param.sfo");
        ui.horizontal(|ui| {
            if ui.add_enabled(changed && !editing && !self.is_saving(), egui::Button::new("Save to file"))
                .on_hover_text(path.display().to_string())
                .on_disabled_hover_text("Apply your edit before saving. Save becomes available when values have changed.")
                .clicked()
            {
                let table = table.clone();
                let ctx = ui.ctx().clone();
                let (sender, receiver) = mpsc::channel();
                self.receiver = Some(receiver);
                self.result = None;
                std::thread::spawn(move || {
                    let result = write_sfo(&path, &table).map(|()| table).map_err(|error| error.to_string());
                    let _ = sender.send(result);
                    ctx.request_repaint();
                });
            }
            if self.is_saving() {
                ui.spinner();
                ui.label("Saving…");
            } else if editing {
                ui.label("Editing value");
            } else if changed {
                ui.label("Unsaved changes");
            } else if matches!(self.result, Some(Ok(()))) {
                ui.label("Saved");
            }
        });
        if let Some(Err(error)) = &self.result {
            ui.colored_label(
                ui.visuals().error_fg_color,
                format!("Could not save SFO: {error}"),
            );
        }
    }
}
