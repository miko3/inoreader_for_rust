mod domain;
mod infrastructure;
mod repositories;
mod services;

use dotenv::dotenv;
use infrastructure::InoreaderClient;
use services::authentication_service::AuthenticationService;
use services::token_service::TokenService;
use std::env;
use std::path::Path;
use std::process::exit;

use crate::repositories::articles_data_repository::ArticlesDataRepository;
use crate::repositories::config_repository::ConfigRepository;

const CONFIG_FILE_NAME: &str = ".config";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let client_id = env::var("INOREADER_CLIENT_ID").expect("INOREADER_CLIENT_ID is not set");
    let client_secret =
        env::var("INOREADER_CLIENT_SECRET").expect("INOREADER_CLIENT_SECRET is not set");

    match args.get(1).map(String::as_str) {
        Some("setup") => {
            if exist_token_file() {
                eprintln!("Token file already exists.");
                exit(1);
            }

            let redirect_uri =
            env::var("INOREADER_REDIRECT_URI").expect("INOREADER_REDIRECT_URI is not set");

            AuthenticationService::app_authenticate(&client_id, &client_secret, &redirect_uri)
            .await
            .expect("Authentication failed");
        }
        Some("fetch_stream") => {
            if !exist_token_file() {
                eprintln!("Token file not found. Please run 'setup' command first.");
                exit(1);
            }

            let config_repository = ConfigRepository::new(CONFIG_FILE_NAME);
            let refresh_token = config_repository.get_saved_refresh_token();

            if config_repository.is_token_expired() {

                let _authentication_service = TokenService::new(&client_id, &client_secret, &refresh_token)
                    .refreshing_token()
                    .await
                    .expect("Failed to refresh token");
            }

            let access_token = config_repository.get_saved_access_token();
            let client = InoreaderClient::new(client_id, client_secret, access_token.to_string());
            let articles_data = client
                .fetch_stream_contents()
                .await
                .expect("Failed to fetch contents");

            ArticlesDataRepository::save_articles_to_csv(&articles_data)
                .expect("Failed to save articles to csv");

            println!("Done!");
        }
        _ => {
            eprintln!("Invalid command. Use 'setup' or 'fetch_stream'.");
        }
    }
}

fn exist_token_file() -> bool {
    let token_path = Path::new(".config");
    token_path.exists()
}