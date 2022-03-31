use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub token: String,
}
