use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::domain::{ApiResponse, Config, TokenResponse};
use crate::repository::ConfigRepository;

pub struct AuthenticationService;

pub struct ResponseParser;

impl AuthenticationService {
    fn generate_random_state() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    pub async fn get_access_token(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let response = reqwest::Client::new()
            .post("https://www.inoreader.com/oauth2/token")
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("redirect_uri", redirect_uri),
                ("code", code),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;

        response.json::<TokenResponse>().await
    }

    pub async fn app_authenticate(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<(), reqwest::Error> {
        let state: String = Self::generate_random_state();
        let auth_url: String = format!("https://www.inoreader.com/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&scope=read&state={}", client_id, redirect_uri, state);
        println!("Please navigate to: {}", auth_url);

        println!("Enter the code from the URL here: ");
        let mut code: String = String::new();
        std::io::stdin().read_line(&mut code).unwrap();
        let code: &str = code.trim();

        let token_response =
            Self::get_access_token(&client_id, &client_secret, &redirect_uri, &code).await?;

        let expires_in_seconds = token_response.expires_in;
        let current_time: SystemTime = SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let expiry_time = since_the_epoch + Duration::new(expires_in_seconds.into(), 0);

        // save authorization code and state
        let app_authorization_info = Config {
            authorization_code: code.to_string(),
            state: state.to_string(),
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: expiry_time.as_secs(),
        };

        let _result = ConfigRepository::save_config(&app_authorization_info);

        Ok(())
    }
}

impl ResponseParser {
    pub fn parse_response(
        json_str: &str,
    ) -> Result<(Vec<(String, String)>, Option<String>), serde_json::Error> {
        let response: ApiResponse = serde_json::from_str(json_str)?;
        let articles: Vec<(String, String)> = response
            .items
            .iter()
            .map(|item| (item.title.clone(), item.canonical[0].href.clone()))
            .collect();

        Ok((articles, response.continuation))
    }
}
