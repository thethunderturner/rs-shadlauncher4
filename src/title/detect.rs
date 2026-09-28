use crate::title::Title;
use crate::title::app::{find_app, find_title_id};
use crate::title::app_union::find_app_union;
use crate::title::patch::find_patch;
use crate::title::sfo::sfo::read_sfo;
use suppaftp::FtpStream;

pub const PFSMNT_PATH: &str = "/mnt/sandbox/pfsmnt";
pub fn current(ftp: &mut FtpStream) -> Vec<Title> {
    let mut titles: Vec<Title> = Vec::new();

    let mounts = match ftp.nlst(Some(PFSMNT_PATH)) {
        Ok(mounts) => mounts,
        Err(error) => {
            println!("Could not read mounted titles: {error}");
            return titles;
        }
    };

    let title_ids = find_title_id(&mounts);
    for title_id in title_ids {
        let app = find_app(&mounts, &title_id).expect("Could not find app");
        let patch = find_patch(&mounts, &title_id);
        let app_union = find_app_union(&mounts, &title_id);

        let sfo = match read_sfo(ftp, &title_id) {
            Ok(sfo) => sfo,

            Err(error) => {
                println!("Could not read param.sfo for {}: {}", title_id, error);

                continue;
            }
        };

        titles.push(Title {
            title_id,
            name: sfo.data_table.find_string("TITLE"),
            version: sfo.data_table.find_string("VERSION"),
            app,
            patch,
            app_union,
            dlcs: Vec::new(),
        });
    }

    titles
}
