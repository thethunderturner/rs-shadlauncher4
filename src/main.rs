use eframe::egui;

pub mod app;
pub mod config;
pub mod devices;
pub mod emulation;
pub mod gui;
pub mod library;
pub mod localization;
pub mod sfo;
pub mod versions;

fn main() -> eframe::Result {
    if let Err(error) = config::storage::initialize_user_directory() {
        eprintln!("Failed to initialize user directory: {error}");
        std::process::exit(1);
    }

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_title("rs-shadLauncher4")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native("rs-shadLauncher4", options, Box::new(|cc| Ok(Box::new(app::App::new(cc)))))
}
