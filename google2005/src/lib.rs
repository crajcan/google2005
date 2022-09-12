use scraper::Html;

pub mod home_page_response;
mod parser;
mod search_request;
mod search_result;
mod search_results;
mod search_results_response;
mod utils;

pub use home_page_response::HomePageResponse;
use search_request::SearchRequest;
use search_results::SearchResults;
use search_results_response::SearchResultsResponse;
pub use utils::google2005_error::Google2005Error;

#[allow(unused_variables)]
pub fn scrape(query: &str, results_page: &str) -> Result<SearchResultsResponse, Google2005Error> {
    println!("in lib, query: {}", query);

    println!("got a response body of length: {}", results_page.len());
    // let response_body = fs::read_to_string("test_seeds/cubs2.html").unwrap();

    // write to file
    // let mut file = fs::File::create("george_clooney.html").unwrap();
    // file.write_all(response_body.as_bytes()).unwrap();
    let request = SearchRequest::new(query);
    let dom = Html::parse_document(&results_page);

    let mut hyperlinks = SearchResults::new(&dom);

    let search_results = hyperlinks.filter();

    let response = SearchResultsResponse::new(&search_results, request)?;

    Ok(response)
}
