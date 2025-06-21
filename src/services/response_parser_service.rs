use crate::domain::ApiResponse;

type ParseResult = Result<(Vec<(String, String)>, Option<String>), serde_json::Error>;

pub struct ResponseParser;

impl ResponseParser {
    pub fn parse_response(json_str: &str) -> ParseResult {
        let response: ApiResponse = serde_json::from_str(json_str)?;
        let articles: Vec<(String, String)> = response
            .items
            .iter()
            .map(|item| (item.title.clone(), item.canonical[0].href.clone()))
            .collect();

        Ok((articles, response.continuation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response() {
        let json_str = r#"{
            "direction":"ltr",
            "id":"user\/-\/state\/com.google\/annotated",
            "title":"Annotated",
            "description":"",
            "self":{
                "href":"https:\/\/www.inoreader.com\/reader\/api\/0\/stream\/contents\/"
            },
            "updated":1618212570,
            "updatedUsec":"1618212570146918",
            "items":[
                {
                    "crawlTimeMsec":"1618211779000",
                    "timestampUsec":"1618211779000000",
                    "id":"tag:google.com,2005:reader\/item\/0000000693c3bc0c",
                    "categories":[
                        "user\/1005921515\/state\/com.google\/reading-list",
                        "user\/1005921515\/state\/com.google\/read",
                        "user\/1005921515\/label\/Tech"
                    ],
                    "title":"Windows and Linux devices are under attack by a new cryptomining worm",
                    "published":1617969599,
                    "updated":1617990787,
                    "canonical":[
                        {
                        "href":"https:\/\/arstechnica.com\/?p=1755573"
                        }
                    ],
                    "alternate":[
                        {
                        "href":"https:\/\/arstechnica.com\/?p=1755573",
                        "type":"text\/html"
                        }
                    ],
                    "summary":{
                        "direction":"ltr",
                        "content":"\u003Cdiv\u003E \n\u003Cimg src=\u0022https:\/\/cdn.arstechnica.net\/wp-content\/uploads\/2021\/04\/enterprise-server-800x545.jpeg\u0022 alt=\u0022Windows and Linux devices are under attack by a new cryptomining worm\u0022\u003E\u003Cp style=\u0022font-size:.8em;\u0022\u003E\u003Ca href=\u0022https:\/\/cdn.arstechnica.net\/wp-content\/uploads\/2021\/04\/enterprise-server.jpeg\u0022\u003EEnlarge\u003C\/a\u003E (credit: Getty Images)\u003C\/p\u003E  \u003Cdiv\u003E\u003Ca\u003E\u003C\/a\u003E\u003C\/div\u003E \n\u003Cp\u003EA newly discovered cryptomining worm is stepping up its targeting of Windows and Linux devices with a batch of new exploits and capabilities, a researcher said.\u003C\/p\u003E \n\u003Cp\u003EResearch company Juniper started monitoring what it’s calling the Sysrv botnet in December. One of the botnet’s malware components was a worm that spread from one vulnerable device to another without requiring any user action. It did this by scanning the Internet for vulnerable devices and, when found, infecting them using a list of exploits that has increased over time.\u003C\/p\u003E \n\u003Cp\u003EThe malware also included a cryptominer that uses infected devices to create the Monero digital currency. There was a separate binary file for each component.\u003C\/p\u003E\u003C\/div\u003E\u003Cp\u003E\u003Ca href=\u0022https:\/\/arstechnica.com\/?p=1755573#p3\u0022\u003ERead 11 remaining paragraphs\u003C\/a\u003E | \u003Ca href=\u0022https:\/\/arstechnica.com\/?p=1755573\u0026amp;comments=1\u0022\u003EComments\u003C\/a\u003E\u003C\/p\u003E"
                    },
                    "author":"Dan Goodin",
                    "likingUsers":[

                    ],
                    "comments":[

                    ],
                    "commentsNum":-1,
                    "annotations":[
                        {
                        "id":1126412668,
                        "start":402,
                        "end":548,
                        "added_on":1618211779,
                        "text":"It did this by scanning the Internet for vulnerable devices and, when found, infecting them using a list of exploits that has increased over time.",
                        "note":"Check your firewall!",
                        "user_id":1005921515,
                        "user_name":"Yordan Yordanov",
                        "user_profile_picture":"https:\/\/www.inoreader.com\/cdn\/profile_picture\/1005921515\/T9zu6Ay6MqMT?s=128"
                        }
                    ],
                    "origin":{
                        "streamId":"feed\/http:\/\/feeds.arstechnica.com\/arstechnica\/gadgets",
                        "title":"Ars Technica » Gear \u0026 Gadgets",
                        "htmlUrl":"http:\/\/arstechnica.com\/"
                    }
                }
            ],
            "continuation":"gmMZgKmmqI4U"
        }"#;

        let (articles, _continuation) = ResponseParser::parse_response(json_str).unwrap();

        assert_eq!(articles.len(), 1);
        assert_eq!(
            articles[0].0,
            "Windows and Linux devices are under attack by a new cryptomining worm"
        );
    }
}
