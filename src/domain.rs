use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    // token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    // scope: String,
}

pub struct Config {
    pub authorization_code: String,
    pub state: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    pub items: Vec<Item>,
    pub continuation: Option<String>,
}

#[derive(Deserialize)]
pub struct Item {
    pub title: String,
    pub canonical: Vec<Link>,
    // alternate: Vec<Link>  // 必要に応じて
}

#[derive(Deserialize)]
pub struct Link {
    pub href: String,
}
