use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::Client;
use services::response_parser_service::ResponseParser;

use crate::services;

const ENDPOINT_URL : &str = "https://www.inoreader.com/reader/api/0";
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub struct InoreaderClient {
    http_client: Client,
    client_id: String,
    client_secret: String,
    access_token: String,
}

impl InoreaderClient {
    pub fn new(client_id: String, client_secret: String, access_token: String) -> Self {
        InoreaderClient {
            http_client: Client::new(),
            client_id,
            client_secret,
            access_token,
        }
    }

    pub async fn fetch_stream_contents(&self) -> Result<Vec<(String, String)>, reqwest::Error> {
        let input: &str = "user/-/state/com.google/starred";
        let encoded: String = utf8_percent_encode(input, FRAGMENT).to_string();
        let mut continuation: Option<String> = Some(String::new());
        let mut counter: i32 = 0;
        let mut all_articles: Vec<(String, String)> = Vec::new();
        let max_iterations: i32 = 10;

        while counter < max_iterations && continuation.is_some() {
            let url: String = format!(
                "{}/stream/contents/{}?AppId={}&AppKey={}&n=100&c={}",
                ENDPOINT_URL,
                encoded,
                &self.client_id,
                &self.client_secret,
                continuation.as_ref().unwrap()
            );

            let response: reqwest::Response = self
                .http_client
                .get(&url)
                .header("Authorization", format!("Bearer {}", &self.access_token))
                .send()
                .await?;

            if response.status().is_success() {
                let content: String = response.text().await?;
                match ResponseParser::parse_response(&content) {
                    Ok((articles, new_continuation)) => {
                        all_articles.extend(articles.iter().cloned());
                        continuation = new_continuation;
                    }
                    Err(e) => {
                        eprintln!("Error parsing response: {}", e);
                        break;
                    }
                }
            } else {
                eprintln!(
                    "Failed to fetch subscriptions. Status: {}",
                    response.status()
                );
                break;
            }
            counter += 1;
            println!("Fetching page {}...", counter)
        }

        Ok(all_articles)
    }
}
