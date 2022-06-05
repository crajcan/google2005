use askama::Template;
use serde::Serialize;
use urlencoding::decode;

use crate::{search_results::SearchResults, utils::google2005_error::Google2005Error};

#[derive(Debug, Serialize)]
struct DecodedResult {
    url: String,
    title: String,
    description: String,
}

#[derive(Debug, Serialize, Template)]
#[template(path = "search.html")]
pub struct SearchResultsResponse {
    results: Vec<DecodedResult>,
    query: String,
}

impl SearchResultsResponse {
    pub fn new(
        parsed: &SearchResults,
        query: &str,
    ) -> Result<SearchResultsResponse, Google2005Error> {
        let mut results: Vec<DecodedResult> = vec![];

        for result in &parsed.results {
            let decoded_url = decode(result.url).unwrap();
            let joined_title = result.title.as_ref().unwrap().join(" ");
            let joined_description = match result.description.as_ref() {
                Some(description) => description.join(" "),
                None => String::from(""),
            };

            results.push(DecodedResult {
                url: decoded_url.to_string(),
                title: joined_title.to_string(),
                description: joined_description.to_string(),
            });
        }

        Ok(SearchResultsResponse {
            results: results,
            query: query.to_string(),
        })
    }
}
