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
pub fn google(query: &str, request_search_from_google: impl Fn(&str) -> Result<String, Google2005Error>) -> Result<SearchResultsResponse, Google2005Error> {
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

