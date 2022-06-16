// use http::{Request, Response};
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

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

// use std::fs;
// use std::io::Write;

#[allow(unused_variables)]
pub fn google(query: &str) -> Result<SearchResultsResponse, Google2005Error> {
    println!("in lib, query: {}", query);

    let response_body = request_search_from_google(query)?;
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

#[allow(dead_code)]
fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let url = format!("https://www.google.com/search?q={}", query);
    let mut resp = Request::get(url).send("example backend")?;

    let body_payload = vec![];
    resp.get_body_mut().write_bytes(&body_payload);

    match resp.get_status() {
        StatusCode::OK => Ok(String::from_utf8(body_payload)?),
        _ => Err(Google2005Error::new(
            None,
            Some("Error requesting search from google"),
        )),
    }
}
