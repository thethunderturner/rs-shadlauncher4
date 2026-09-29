use crate::sfo::data_table::{SfoDataParam, SfoValue};
use eframe::egui;
use std::path::Path;

pub mod compatibility;
mod gui;
pub mod scanning;
mod sfo;

fn main() {
    let res = sfo::sfo::read_sfo(Path::new(
        "/run/media/mateo/ExtremeMainPart/Games/Emulation/dumped_games/ps4/Games/CUSA00127/sce_sys/param.sfo",
    ));
    // println!("{:#?}", res);
    let res1 = scanning::scan(Path::new(
        "/run/media/mateo/ExtremeMainPart/Games/Emulation/dumped_games/ps4/Games/",
    ));
    println!("{:#?}", res1);

    // let options = eframe::NativeOptions {
    //     renderer: eframe::Renderer::Glow,
    //     viewport: egui::ViewportBuilder::default()
    //         .with_title("rs-shadLauncher4")
    //         .with_inner_size([1280.0, 720.0])
    //         .with_min_inner_size([480.0, 320.0])
    //         .with_icon(
    //             eframe::icon_data::from_png_bytes(&include_bytes!("assets/logo.png")[..])
    //                 .expect("Failed to load icon"),
    //         ),
    //     ..Default::default()
    // };
    //
    // eframe::run_native(
    //     "rs-shadLauncher4",
    //     options,
    //     Box::new(|_cc| Ok(Box::new(gui::LauncherApp::default()))),
    // )
}
