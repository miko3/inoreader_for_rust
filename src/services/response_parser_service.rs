use crate::domain::ApiResponse;

pub struct ResponseParser;

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
