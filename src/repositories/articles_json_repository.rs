use std::fs::File;
use std::io::Write;

pub struct ArticlesJsonRepository;

impl ArticlesJsonRepository {
    pub fn save_articles_to_json(articles: &Vec<(String, String)>) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(articles)?;
        let mut file = File::create("articles.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}