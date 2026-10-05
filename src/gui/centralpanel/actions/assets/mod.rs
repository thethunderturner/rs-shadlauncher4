use crate::scanning::Title;
use eframe::egui;
use std::path::Path;

/// egui reads and decodes the PNG in the background, then caches its texture.
pub fn load_icon(title: &Title) -> Option<egui::Image<'static>> {
    title
        .icon_path
        .as_deref()
        .map(|path| load_image(path).fit_to_exact_size(egui::vec2(40.0, 40.0)))
}

pub fn load_background(title: &Title) -> Option<egui::Image<'static>> {
    title.background_path.as_deref().map(load_image)
}

pub fn show_background(ui: &egui::Ui, title: &Title) {
    let Some(image) = load_background(title) else {
        return;
    };
    let rect = ui.available_rect_before_wrap();
    if let Ok(egui::load::TexturePoll::Ready { texture }) =
        image.load_for_size(ui.ctx(), rect.size())
    {
        // Fit the image without stretching, then shade it for readable table text.
        let size =
            texture.size * (rect.width() / texture.size.x).min(rect.height() / texture.size.y);
        let image_rect = egui::Rect::from_center_size(rect.center(), size);
        ui.painter().image(
            texture.id,
            image_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        ui.painter()
            .rect_filled(rect, 0.0, ui.visuals().panel_fill.gamma_multiply(0.85));
    }
}

pub fn forget_background(ctx: &egui::Context, title: &Title) {
    if let Some(path) = &title.background_path {
        ctx.forget_image(&image_uri(path));
    }
}

fn load_image(path: &Path) -> egui::Image<'static> {
    egui::Image::new(image_uri(path))
}

fn image_uri(path: &Path) -> String {
    format!("file://{}", path.display())
}

/// Allow Refresh List to reload files which changed on disk.
pub fn clear_cache(ctx: &egui::Context, titles: &[Title]) {
    for title in titles {
        for path in [&title.icon_path, &title.background_path]
            .into_iter()
            .flatten()
        {
            ctx.forget_image(&image_uri(path));
        }
    }
}
