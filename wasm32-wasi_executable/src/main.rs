use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Body, Error, Request, Response};
extern crate google2005;
use google2005::Google2005Error;

use std::intrinsics::write_bytes;

use fastly::http::header::USER_AGENT;
// use http::{Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_method() {
        &Method::GET | &Method::HEAD => (),

        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    if req.get_path() == "/search" {
        match Google2005Response::new(req.get_query_str())
            .contents()
            .as_str()
        {
            "" => Ok(Response::from_status(StatusCode::UNPROCESSABLE_ENTITY)
                .with_body_text_plain("Please enter a query.")),
            resp => Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(resp)),
        }
    } else {
        Ok(
            Response::from_status(StatusCode::NOT_FOUND).with_body_text_plain(&format!(
                r#"The requested page: "{}", could not be found\n"#,
                req.get_url_str()
            )),
        )
    }
}

use askama::Template;

const SEARCH_URI: &'static str = "q=";
pub struct Google2005Response {
    contents: String,
}

impl Google2005Response {
    pub fn new(query: Option<&str>) -> Google2005Response {
        if query == None {
            return Google2005Response {
                contents: String::from(""),
            };
        }

        let query = query.unwrap();

        if !query.starts_with(SEARCH_URI) {
            return Google2005Response {
                contents: "".to_string(),
            };
        }

        let query = query.trim_start_matches(SEARCH_URI);

        match Self::html_search_response(query) {
            Ok(contents) => Google2005Response { contents },
            Err(e) => {
                println!(
                    "returning error to front end: \n\n{}\n\nthat's the error",
                    e
                );
                Google2005Response {
                    contents: format!("{}", e),
                }
            }
        }
    }

    fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
        let search_results = google2005::scrape(query, &request_search_from_google(query)?)?;

        Ok(search_results.render()?)
    }

    fn contents(self) -> String {
        self.contents
    }
}

// struct Google2005Request;

// impl Google2005Request {
//     pub fn query(query: &str) -> String {
//         let bytes = query.as_bytes();
//         let after_equals = &bytes[14..];
//         let until_space = after_equals.split(|c| *c == b' ').next().unwrap();
//         let string_query = String::from_utf8_lossy(until_space);
//         string_query.to_string()
//     }
// }

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";
const GOOGLE2005LAMBDA: &str =
    "https://gwc19qn2w3.execute-api.us-east-2.amazonaws.com/google2005lambda";

// Try to clone version 21 of the google(lambda) backend and implement TLS
#[allow(dead_code)]
pub fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let url = format!("http://www.google.com/search?q={}", query);
    let url_copy = url.clone();

    let mut request = Request::post(GOOGLE2005LAMBDA)
        .with_header("Content-Type", "application/json")
        .with_header("Accept", "*/*")
        .with_header("Host", "gwc19qn2w3.execute-api.us-east-2.amazonaws.com")
        .with_header("User-Agent", USER_AGENT_STRING)
        .with_body(request_body(url));
    println!("request_body: {:#?}", request_body(url_copy).into_string());

    println!("request: {:#?}", request);
    let mut resp = request.send("google")?;
    println!("response: {:#?}", resp);

    let body = resp.take_body().into_string();
    // println!("************** HTTP status: {:?}", resp.get_status());
    // println!("body: {}", body);

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
