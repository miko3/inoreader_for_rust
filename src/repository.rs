use crate::domain::Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub struct ConfigRepository;

impl ConfigRepository {
    pub fn load_config(path: &Path) -> HashMap<String, String> {
        // ファイルから設定を読み込むロジック
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

    pub fn save_config(config: &Config) -> io::Result<()> {
        // ファイルに設定を保存するロジック
        let mut file = File::create(".config")?;
        writeln!(file, "AuthorizationCode:{}", config.authorization_code)?;
        writeln!(file, "State:{}", config.state)?;
        writeln!(file, "AccessToken:{}", config.access_token)?;
        writeln!(file, "RefreshToken:{}", config.refresh_token)?;
        writeln!(file, "ExpiresIn:{}", config.expires_in)?;

        Ok(())
    }

    pub fn save_articles_to_json(articles: &Vec<(String, String)>) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(articles)?;
        let mut file = File::create("articles.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
