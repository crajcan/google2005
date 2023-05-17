use std::f32::consts::E;

use crate::utils::request;
use google2005::Google2005Error;
use reqwest::Client;

const SEARCH_URI: &'static str = "GET /search?q=";

pub struct Response {
    contents: String,
    status_line: String,
    additional_headers: Option<String>,
}

impl Response {
    pub async fn new(buffer: &[u8]) -> Response {
        if !buffer.starts_with(SEARCH_URI.as_bytes()) {
            return Response {
                contents: "".to_string(),
                status_line: format!("HTTP/1.1 404 Not Found"),
                additional_headers: None,
            };
        }

        let query = request::query(&buffer);

        match Self::html_search_response(&query).await {
            Ok(contents) => Response {
                contents,
                status_line: "HTTP/1.1 200 OK".to_string(),
                additional_headers: None,
            },
            Err(e) => Response {
                contents: format!("{}", e),
                status_line: "HTTP/1.1 302 Found".to_string(),
                additional_headers: Some(format!(
                    "Location: {}\r\n",
                    Self::google_url(&query)
                )),
            },
        }
    }

    async fn html_search_response(
        query: &str,
    ) -> Result<String, google2005::Google2005Error> {
        let search_results = google2005::scrape(
            query,
            &Self::request_search_from_google(query).await?,
        );

        match search_results {
            Ok(results) => Ok(results.to_string()),
            Err(e) => Err(e),
        }
    }

    fn google_url(query: &str) -> String {
        format!("https://www.google.com/search?q={}", query)
    }

    #[allow(dead_code)]
    async fn request_search_from_google(
        query: &str,
    ) -> Result<String, Google2005Error> {
        let client = Client::new();
        let url = Self::google_url(query);

        let res = match client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                return Err(Google2005Error::new(
                    None,
                    Some(&format!(
                        "Could not retrieve page from google: {}",
                        e
                    )),
                ))
            }
        };

        let body = match res.text().await {
            Ok(body) => body,
            Err(e) => {
                return Err(Google2005Error::new(
                    None,
                    Some(&format!("Could not parse page from response: {}", e)),
                ))
            }
        };

        Ok(body)
    }

    pub fn render(&self) -> String {
        "Content-Type: text/html; charset=UTF-8\r\nServer: Google2005\r\n";
        format!(
            "{}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_line,
            match self.additional_headers {
                Some(ref s) => s,
                None => "",
            },
            self.contents.len(),
            self.contents
        )
    }
}
