use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::domain::{Config, TokenResponse};
use crate::repositories::config_repository::ConfigRepository;

pub struct AuthenticationService;

const AUTH_URL : &str = "https://www.inoreader.com/oauth2/auth";
const TOKEN_URL: &str = "https://www.inoreader.com/oauth2/token";

impl AuthenticationService {
    fn generate_random_state() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    async fn get_access_token(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let response = reqwest::Client::new()
            .post(TOKEN_URL)
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
        let auth_url: String = format!("{}?client_id={}&redirect_uri={}&response_type=code&scope=read&state={}", AUTH_URL, client_id, redirect_uri, state);
        println!("Please navigate to: {}", auth_url);

        println!("Enter the code from the URL here: ");
        let mut code: String = String::new();
        std::io::stdin().read_line(&mut code).unwrap();
        let code: &str = code.trim();

        let token_response: TokenResponse =
            Self::get_access_token(&client_id, &client_secret, &redirect_uri, &code).await?;

        let expiry_time = Self::calculate_expiry_time(token_response.expires_in);

        // save authorization code and state
        let app_authorization_info = Config {
            authorization_code: code.to_string(),
            state: state.to_string(),
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: expiry_time,
        };

        let config_repository = ConfigRepository::new(".config");
        let _result = config_repository.save_config(&app_authorization_info);

        Ok(())
    }

    fn calculate_expiry_time(expires_in_seconds: u64) -> u64 {
        let current_time: SystemTime = SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let expiry_time = since_the_epoch + Duration::new(expires_in_seconds.into(), 0);
        expiry_time.as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_state() {
        let state = AuthenticationService::generate_random_state();
        assert_eq!(state.len(), 30);
    }

    #[test]
    fn test_calculate_expiry_time() {
        let expiry_time = AuthenticationService::calculate_expiry_time(3600);
        let current_time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        assert_eq!(expiry_time - current_time, 3600);
    }
}