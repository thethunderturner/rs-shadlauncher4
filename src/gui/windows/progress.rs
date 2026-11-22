use eframe::egui;

#[derive(Clone)]
pub struct Progress {
    pub title: String,
    pub fraction: Option<f32>,
    pub message: String,
}

pub fn show(ctx: &egui::Context, progress: &Progress) {
    egui::Window::new(progress.title.clone())
        .id(egui::Id::unique("library_progress"))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .default_width(360.0)
        .show(ctx, |ui| {
            ui.label(&progress.message);
            let bar = match progress.fraction {
                Some(fraction) => egui::ProgressBar::new(fraction).show_percentage(),
                None => egui::ProgressBar::new(0.0).animate(true).text("Please wait"),
            };
            ui.add(bar.desired_width(ui.available_width()));
        });
}
