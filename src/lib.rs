use fastly::http::header::USER_AGENT;
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

#[allow(dead_code)]
fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let url = format!("https://www.google.com/search?q={}", query);

    let request = Request::get(url)
        .with_header("Host", "www.google.com")
        .with_header("User-Agent", USER_AGENT_STRING)
        .with_header("Accept", "*/*");

    println!("request: {:#?}", request);

    println!("about to send a request");
    let mut resp = request.send("google")?;
    // println!("got the response: {:#?}", resp);

    let body = resp.take_body().into_string();
    println!("************** HTTP status: {:?}", resp.get_status());
    // println!("************** got the body too: {:?}", body);

    match resp.get_status() {
        StatusCode::OK => Ok(body),
        _ => Err(Google2005Error::new(
            None,
            Some("Error requesting search from google"),
        )),
    }
}
