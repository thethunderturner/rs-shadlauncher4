use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Users {
    commit_hash: String,
    user: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    np_age: u8,
    np_country: String,
    np_date_of_birth: String,
    np_language: String,
    player_index: u8,
    shadnet_email: String,
    shadnet_enabled: bool,
    shadnet_npid: String,
    shadnet_password: String,
    shadnet_token: String,
    user_color: u8,
    user_id: u32,
    user_name: String,
}

impl Default for Users {
    fn default() -> Self {
        let user = (0..4)
            .map(|index| User {
                player_index: index + 1,
                user_color: index + 1,
                user_id: 1000 + u32::from(index),
                user_name: if index == 0 {
                    "shadps4".to_owned()
                } else {
                    format!("shadps4-{index}")
                },
                ..User::default()
            })
            .collect();

        Self {
            commit_hash: String::new(),
            user,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            np_age: 30,
            np_country: String::from("us"),
            np_date_of_birth: String::from("1994-01-01"),
            np_language: String::from("en"),
            player_index: 1,
            shadnet_email: String::new(),
            shadnet_enabled: false,
            shadnet_npid: String::new(),
            shadnet_password: String::new(),
            shadnet_token: String::new(),
            user_color: 1,
            user_id: 1000,
            user_name: String::from("shadps4"),
        }
    }
}
