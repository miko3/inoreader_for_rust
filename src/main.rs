mod domain;
mod infrastructure;
mod repository;
mod service;

use dotenv::dotenv;
use infrastructure::InoreaderClient;
use repository::ConfigRepository;
use service::AuthenticationService;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token_path = Path::new(".config");
    let client_id = env::var("INOREADER_CLIENT_ID").expect("INOREADER_CLIENT_ID is not set");
    let client_secret =
        env::var("INOREADER_CLIENT_SECRET").expect("INOREADER_CLIENT_SECRET is not set");
    let redirect_uri =
        env::var("INOREADER_REDIRECT_URI").expect("INOREADER_REDIRECT_URI is not set");

    if !token_path.exists() {
        AuthenticationService::app_authenticate(&client_id, &client_secret, &redirect_uri)
            .await
            .expect("Authentication failed");
    }

    let config = ConfigRepository::load_config(token_path);
    let access_token = config
        .get("AccessToken")
        .expect("AccessToken not found in token file");

    let client = InoreaderClient::new(client_id, client_secret, access_token.to_string());
    let _result = client
        .fetch_stream_contents()
        .await
        .expect("Failed to fetch subscriptions");
}
