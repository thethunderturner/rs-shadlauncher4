use eframe::egui;
use std::path::Path;

pub mod compatibility;
mod gui;
pub mod scanning;
mod sfo;

fn main() -> eframe::Result {
    let games_path = Path::new("/run/media/mateo/SEAGATE 4TB/ps4/Games/").to_path_buf();

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
        Box::new(move |cc| Ok(Box::new(gui::LauncherApp::new(cc, games_path)))),
    )
}
