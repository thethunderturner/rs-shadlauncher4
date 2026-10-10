use serde_json::Value;
use crate::gui::menubar::settings::{Settings};
use crate::gui::menubar::settings::helpers::audio_devices::AudioDevices;

impl Settings {
    pub fn load(&mut self) {
        self.error = None;
        self.draft = None;
        match crate::common::user_directory() {
            Ok(dir) => {
                let path = dir.join("config.json");
                match std::fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|text| {
                        serde_json::from_str::<Value>(&text).map_err(|e| e.to_string())
                    }) {
                    Ok(mut value) if value.is_object() => {
                        normalize_sections(&mut value);
                        if value
                            .get("Frontend")
                            .is_some_and(|frontend| !frontend.is_object())
                        {
                            self.error =
                                Some("Frontend settings must be a JSON object.".to_owned());
                        } else if let Err(error) =
                            serde_json::from_value::<crate::common::config::Config>(value.clone())
                        {
                            self.error = Some(format!("Could not read config.json: {error}"));
                        } else {
                            self.audio_devices = AudioDevices::scan(
                                value["Audio"]["audio_backend"].as_u64().unwrap_or(0),
                            );
                            self.draft = Some(value);
                        }
                    }
                    Ok(_) => {
                        self.error = Some("config.json must contain a JSON object.".to_owned())
                    }
                    Err(error) => self.error = Some(format!("Could not load config.json: {error}")),
                }
                self.config_path = Some(path);
            }
            Err(error) => self.error = Some(format!("Could not locate user directory: {error}")),
        }
    }

    pub fn save(&mut self) -> bool {
        let (Some(path), Some(draft)) = (&self.config_path, &self.draft) else {
            return false;
        };
        let result = serde_json::from_value::<crate::common::config::Config>(draft.clone())
            .map_err(|error| error.to_string())
            .and_then(|_| serde_json::to_vec_pretty(draft).map_err(|error| error.to_string()))
            .and_then(|mut bytes| {
                bytes.push(b'\n');
                std::fs::write(path, bytes).map_err(|error| error.to_string())
            });
        match result {
            Ok(()) => {
                self.error = None;
                true
            }
            Err(error) => {
                self.error = Some(format!("Could not save config.json: {error}"));
                false
            }
        }
    }
}

fn normalize_sections(config: &mut Value) {
    let Some(root) = config.as_object_mut() else {
        return;
    };
    for (old, new) in [
        ("audio", "Audio"),
        ("debug", "Debug"),
        ("gpu", "GPU"),
        ("general", "General"),
        ("input", "Input"),
        ("log", "Log"),
        ("network", "Network"),
        ("vulkan", "Vulkan"),
    ] {
        if let Some(value) = root.remove(old) {
            if let Some(existing) = root.get_mut(new) {
                if let (Some(old_fields), Some(new_fields)) =
                    (value.as_object(), existing.as_object_mut())
                {
                    for (key, value) in old_fields {
                        new_fields.entry(key).or_insert_with(|| value.clone());
                    }
                }
            } else {
                root.insert(new.to_owned(), value);
            }
        }
    }
}