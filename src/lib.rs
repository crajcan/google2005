use std::intrinsics::write_bytes;

use fastly::http::header::USER_AGENT;
// use http::{Request, Response};
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Body, Error, Request, Response};

use scraper::Html;

mod parser;
mod search_request;
mod search_result;
mod search_results;
mod search_results_response;
mod utils;

use search_request::SearchRequest;
use search_results::SearchResults;
use search_results_response::SearchResultsResponse;
pub use utils::google2005_error::Google2005Error;

use crate::utils::google2005_error;

// use std::fs;
// use std::io::Write;

#[allow(unused_variables)]
pub fn google(query: &str) -> Result<SearchResultsResponse, Google2005Error> {
    println!("in lib, query: {}", query);

    let response_body = request_search_from_google(query)?;
    println!("got a response body of length: {}", response_body.len());
    // let response_body = fs::read_to_string("test_seeds/cubs2.html").unwrap();

    // write to file
    // let mut file = fs::File::create("george_clooney.html").unwrap();
    // file.write_all(response_body.as_bytes()).unwrap();
    let request = SearchRequest::new(query);
    let dom = Html::parse_document(&response_body);

    let mut hyperlinks = SearchResults::new(&dom);

    let search_results = hyperlinks.filter();

    let response = SearchResultsResponse::new(&search_results, request)?;

    Ok(response)
}

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";
const GOOGLE2005LAMBDA: &str =
    "https://gwc19qn2w3.execute-api.us-east-2.amazonaws.com/google2005lambda";

// Try to clone version 21 of the google(lambda) backend and implement TLS
#[allow(dead_code)]
fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let url = format!("http://www.google.com/search?q={}", query);

    let mut request = Request::post(GOOGLE2005LAMBDA)
        .with_header("Content-Type", "application/json")
        .with_header("Accept", "*/*")
        .with_header("Host", "gwc19qn2w3.execute-api.us-east-2.amazonaws.com")
        .with_body(request_body(url));

    let mut resp = request.send("google")?;

    let body = resp.take_body().into_string();
    println!("************** HTTP status: {:?}", resp.get_status());
    println!("body: {}", body);

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
