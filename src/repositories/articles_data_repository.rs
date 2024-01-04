use std::fs::File;
use std::io::Write;

pub struct ArticlesDataRepository;

impl ArticlesDataRepository {
    pub fn save_articles_to_json(articles: &Vec<(String, String)>) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(articles)?;
        let mut file = File::create("articles.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn save_articles_to_csv(articles: &Vec<(String, String)>) -> Result<(), std::io::Error> {
        let mut file = File::create("articles.csv")?;
        file.write_all("title,url\n".as_bytes())?;

        for (title, url) in articles {
            let escaped_title = title.replace("\"", "\"\"");
            file.write_all(format!("\"{}\",\"{}\"\n", escaped_title, url).as_bytes())?;
        }
        Ok(())
    }
}