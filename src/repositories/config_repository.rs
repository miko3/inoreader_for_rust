use crate::CONFIG_FILE_NAME;
use crate::domain::Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct ConfigRepository {
    config_path: String,
    access_token: String,
    state: String,
    refresh_token: String,
    expires_in: u64,
}

impl ConfigRepository {
    pub fn new(config_path: &str) -> Self {
        let config = Self::load_config(CONFIG_FILE_NAME);

        if config.is_empty() {
            return ConfigRepository {
                config_path: config_path.to_string(),
                access_token: "".to_string(),
                state: "".to_string(),
                refresh_token: "".to_string(),
                expires_in: 0,
            };
        }

        ConfigRepository {
            config_path: config_path.to_string(),
            access_token: config.get("AccessToken").unwrap().to_string(),
            state: config.get("State").unwrap().to_string(),
            refresh_token: config.get("RefreshToken").unwrap().to_string(),
            expires_in: config.get("ExpiresIn").unwrap().parse::<u64>().unwrap(),
        }
    }

    fn load_config(config_path: &str) -> HashMap<String, String> {
        let path: &Path = Path::new(config_path);
        let mut settings = HashMap::new();
        if path.exists() {
            let mut file = File::open(path).expect("Failed to open token file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read token file");

            for line in contents.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    settings.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
        settings
    }

    pub fn save_config(&self, config: &Config) -> io::Result<()> {
        // ファイルに設定を保存するロジック
        let config_file_path = Path::new(&self.config_path);
        let mut file = File::create(config_file_path)?;
        writeln!(file, "AuthorizationCode:{}", config.authorization_code)?;
        writeln!(file, "State:{}", config.state)?;
        writeln!(file, "AccessToken:{}", config.access_token)?;
        writeln!(file, "RefreshToken:{}", config.refresh_token)?;
        writeln!(file, "ExpiresIn:{}", config.expires_in)?;

        Ok(())
    }

    pub fn get_saved_access_token(&self) -> String {
        return self.access_token.to_string();
    }

    pub fn is_token_expired(&self) -> bool {
        let current_time = SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let current_time_in_seconds = since_the_epoch.as_secs();

        if current_time_in_seconds > self.expires_in {
            return true;
        }
        return false;
    }

    pub fn get_saved_refresh_token(&self) -> String {
        return self.refresh_token.to_string();
    }

    pub fn get_saved_state(&self) -> String {
        return self.state.to_string();
    }

    pub fn get_saved_authorization_code(&self) -> String {
        // Note: Authorization code is typically used only once during initial setup
        // This method returns empty string as the code is not stored after token exchange
        String::new()
    }
}