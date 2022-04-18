use askama::Template;
use serde::Serialize;
use urlencoding::decode;

use crate::{search_results::SearchResults, utils::google2005_error::Google2005Error};

#[derive(Debug, Serialize)]
struct DecodedResult {
    url: String,
    title: String,
}

#[derive(Debug, Serialize, Template)]
#[template(path = "search.html")]
pub struct SearchResultsResponse {
    results: Vec<DecodedResult>,
}

impl SearchResultsResponse {
    pub fn new(parsed: &SearchResults) -> Result<SearchResultsResponse, Google2005Error> {
        let mut results: Vec<DecodedResult> = vec![];

        for result in &parsed.results {
            let decoded_url = decode(result.url).unwrap();
            let joined_title = result.title.as_ref().unwrap().join(" ");

            results.push(DecodedResult {
                url: decoded_url.to_string(),
                title: joined_title.to_string(),
            });
        }

        Ok(SearchResultsResponse { results: results })
    }
}
