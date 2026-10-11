use crate::common::config::Config;
use crate::gui::menubar::settings::Settings;
use crate::gui::menubar::settings::helpers::audio_devices::AudioDevices;
use serde_json::Value;

impl Settings {
    pub fn load(&mut self) {
        self.error = None;
        self.draft = None;
        self.original = None;
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
                        } else {
                            match serde_json::from_value::<Config>(value.clone()) {
                                Ok(config) => {
                                    self.audio_devices =
                                        AudioDevices::scan(config.audio.audio_backend as u64);
                                    self.original = Some(value);
                                    self.draft = Some(config);
                                }
                                Err(error) => {
                                    self.error =
                                        Some(format!("Could not read config.json: {error}"))
                                }
                            }
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
        let result = serde_json::to_value(draft)
            .map_err(|error| error.to_string())
            .and_then(|known| {
                let mut merged = self
                    .original
                    .clone()
                    .unwrap_or_else(|| serde_json::json!({}));
                merge_known(&mut merged, known);
                serde_json::to_vec_pretty(&merged).map_err(|error| error.to_string())
            })
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
        ("frontend", "Frontend"),
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

fn merge_known(original: &mut Value, known: Value) {
    match (original, known) {
        (Value::Object(original), Value::Object(known)) => {
            for (key, value) in known {
                merge_known(original.entry(key).or_insert(Value::Null), value);
            }
        }
        (original, known) => *original = known,
    }
}

#[cfg(test)]
mod tests {
    use super::{merge_known, normalize_sections};
    use crate::common::config::Config;
    use serde_json::{Value, json};

    #[test]
    fn typed_edit_preserves_unrecognized_config_keys() {
        let mut original = json!({
            "network": {"shad_net_enabled": false, "future_option": 42},
            "CustomSection": {"keep": true}
        });
        normalize_sections(&mut original);
        let mut config: Config = serde_json::from_value(original.clone()).unwrap();
        config.network.shad_net_enabled = true;
        merge_known(&mut original, serde_json::to_value(config).unwrap());

        assert_eq!(original["Network"]["shad_net_enabled"], Value::Bool(true));
        assert_eq!(original["Network"]["future_option"], json!(42));
        assert_eq!(original["CustomSection"]["keep"], Value::Bool(true));
        assert!(original.get("network").is_none());
    }
}
