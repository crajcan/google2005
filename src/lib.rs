use reqwest::Client;
use scraper::Html;

mod parser;
mod search_result;
mod search_results;
mod search_results_response;
mod utils;

use search_results::SearchResults;
use search_results_response::SearchResultsResponse;
pub use utils::google2005_error::Google2005Error;

// use std::fs;
// use std::io::Write;

#[allow(unused_variables)]
pub async fn google(query: &str) -> Result<SearchResultsResponse, Google2005Error> {
    let response_body = request_search_from_google(query).await?;
    // let response_body = fs::read_to_string("cubs.html").unwrap();

    // write to file
    // let mut file = fs::File::create("cubs.html").unwrap();
    // file.write_all(response_body.as_bytes()).unwrap();

    let dom = Html::parse_document(&response_body);

    let mut hyperlinks = SearchResults::new(&dom);

    let search_results = hyperlinks.filter();

    let response = SearchResultsResponse::new(&search_results, query)?;

    Ok(response)
}

#[allow(dead_code)]
async fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let client = Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
