use eframe::egui;
mod gui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_title("rs-shadLauncher4")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([480.0, 320.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("assets/logo.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "rs-shadLauncher4",
        options,
        Box::new(|_cc| Ok(Box::new(gui::LauncherApp::default()))),
    )
}
