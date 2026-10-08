use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Users {
    commit_hash: String,
    user: Vec<User>,
}

#[derive(Debug, Deserialize)]
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
