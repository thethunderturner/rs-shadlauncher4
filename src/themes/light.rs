use eframe::egui;
use eframe::egui::Visuals;

pub fn light() -> Visuals {
    let mut visuals = Visuals::light();
    let [r, g, b, _] = visuals.selection.bg_fill.to_srgba_unmultiplied();
    visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(r, g, b, 153);
    let [r, g, b, _] = visuals.widgets.hovered.bg_fill.to_srgba_unmultiplied();
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(r, g, b, 153);
    visuals
}
