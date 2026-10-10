use crate::gui::sidepanel::Sidepanel;
use std::path::PathBuf;

impl Sidepanel {
    pub fn games_paths(&self) -> &[PathBuf] {
        &self.games_paths
    }

    pub fn can_refresh(&self) -> bool {
        !self.games_paths.is_empty()
    }

    pub fn load_paths(&mut self) -> bool {
        match crate::common::config::Config::game_paths() {
            Ok(paths) => {
                self.error = None;
                if self.games_paths == paths {
                    return false;
                }
                self.games_paths = paths;
                true
            }
            Err(error) => {
                self.error = Some(format!(
                    "Could not load game folders from config.json: {error}"
                ));
                false
            }
        }
    }
}
