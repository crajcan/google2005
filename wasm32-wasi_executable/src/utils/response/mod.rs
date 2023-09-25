use askama::Template;
use fastly::http::StatusCode;
use fastly::Request as FastlyRequest;
use fastly::Response as FastlyResponse;
use fastly::{mime, Body};
use google2005::Google2005Error;

const SEARCH_URI: &'static str = "q=";
pub struct Response {
    contents: String,
    status: StatusCode,
    additional_headers: Option<(String, String)>,
}

impl Response {
    pub fn new(query: Option<&str>) -> Response {
        if query == None {
            return Response {
                contents: String::from("Please enter a query"),
                status: StatusCode::NOT_FOUND,
                additional_headers: None,
            };
        }

        let query = query.unwrap();

        if !query.starts_with(SEARCH_URI) {
            return Response {
                contents: "".to_string(),
                status: StatusCode::NOT_FOUND,
                additional_headers: None,
            };
        }

        let query = query.trim_start_matches(SEARCH_URI);

        match Self::html_search_response(query) {
            Ok(contents) => Response {
                contents,
                status: StatusCode::OK,
                additional_headers: None,
            },
            Err(e) => Response {
                contents: format!("{}", e),
                status: StatusCode::FOUND,
                additional_headers: Some((
                    "Location".to_string(),
                    google_url(query),
                )),
            },
        }
    }

    fn html_search_response(
        query: &str,
    ) -> Result<String, google2005::Google2005Error> {
        println!("******* requesting search from google *******");
        let results_page = request_search_from_google(query)?;

        println!("******* about to scrape search results *******");
        let search_results = google2005::scrape(query, &results_page)?;
        println!("******* Scraped search results *******");

        Ok(search_results.render()?)
    }

    pub fn render(&self) -> FastlyResponse {
        match self {
            Response {
                status: StatusCode::OK,
                contents: _,
                additional_headers: _,
            } => FastlyResponse::from_status(self.status)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(self.contents.clone()),
            Response {
                status: StatusCode::FOUND,
                contents: _,
                additional_headers: Some((header_name, header_value)),
            } => FastlyResponse::from_status(self.status)
                .with_header(header_name, header_value),
            _ => FastlyResponse::from_status(self.status),
        }
    }
}

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";
const GOOGLE2005LAMBDA: &str =
    "https://gwc19qn2w3.execute-api.us-east-2.amazonaws.com/google2005lambda";

fn google_url(query: &str) -> String {
    format!("https://www.google.com/search?q={}", query)
}

// Try to clone version 21 of the google(lambda) backend and implement TLS
#[allow(dead_code)]
pub fn request_search_from_google(
    query: &str,
) -> Result<String, Google2005Error> {
    let url = google_url(query);
    let url_copy = url.clone();
    println!("my body: {:#?}", request_body(url_copy.clone()).into_string());
    let body = request_body(url_copy.clone());

    let request = FastlyRequest::post(GOOGLE2005LAMBDA)
        .with_header("Content-Type", "application/json")
        .with_header("Accept", "*/*")
        .with_header("Host", "gwc19qn2w3.execute-api.us-east-2.amazonaws.com")
        .with_header("User-Agent", USER_AGENT_STRING)
        .with_body(request_body(url));
    println!("request_body: {:#?}", request_body(url_copy).into_string());

    println!("request: {:#?}", request);
    let mut resp = request.send("google")?;
    println!("****** received response *****");
    println!("response: {:#?}", resp);

    let body = resp.take_body().into_string();

    println!("****** matching on response status *****");
    match resp.get_status() {
        StatusCode::OK => Ok(body),
        _ => Err(Google2005Error::new(
            None,
            Some("Error requesting search from google"),
        )),
    }
}

fn request_body(url: String) -> Body {
    let mut body = Body::new();

    body.write_bytes(request_body_string(url).as_bytes());

    body
}

fn request_body_string(url: String) -> String {
    format!(r#"{{"path":"{}"}}"#, url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request_body_string_adds_key() {
        let url = "https://www.google.com/search?q=george+clooney".to_string();

        assert_eq!(
            r#"{"path":"https://www.google.com/search?q=george+clooney"}"#,
            request_body_string(url)
        )
    }
}
