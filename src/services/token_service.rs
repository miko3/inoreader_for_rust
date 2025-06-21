use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::domain::{Config, TokenResponse};
use crate::repositories::config_repository::ConfigRepository;

pub struct TokenService {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl TokenService {
    pub fn new(client_id: &str, client_secret: &str, refresh_token: &str) -> Self {
        TokenService {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    async fn refresh_access_token(&self) -> Result<TokenResponse, reqwest::Error> {
        let response = reqwest::Client::new()
            .post("https://www.inoreader.com/oauth2/token")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("refresh_token", &self.refresh_token),
                ("grant_type", &"refresh_token".to_string()),
            ])
            .send()
            .await?;

        response.json::<TokenResponse>().await
    }

    pub async fn refreshing_token(&self) -> Result<(), reqwest::Error> {
        let config_repository = ConfigRepository::new(".config");
        let token_response = self.refresh_access_token().await?;

        let expiry_time = Self::calculate_expiry_time(token_response.expires_in);

        let config = Config {
            authorization_code: config_repository.get_saved_authorization_code(),
            state: config_repository.get_saved_state(),
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: expiry_time,
        };

        let config_repository = ConfigRepository::new(".config");
        config_repository
            .save_config(&config)
            .expect("Failed to save token file");

        Ok(())
    }

    fn calculate_expiry_time(expires_in_seconds: u64) -> u64 {
        let current_time: SystemTime = SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let expiry_time = since_the_epoch + Duration::new(expires_in_seconds, 0);
        expiry_time.as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_expiry_time() {
        let expiry_time = TokenService::calculate_expiry_time(3600);
        let current_time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        assert_eq!(expiry_time - current_time, 3600);
    }
}
