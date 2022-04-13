use askama::Template;
use serde::Serialize;
use urlencoding::decode;

use crate::{search_result::SearchResult, utils::google2005_error::Google2005Error};

#[derive(Debug, Serialize)]
struct DecodedResult {
    url: String,
    title: String,
}

#[derive(Debug, Serialize, Template)]
#[template(path = "search.html")]
pub struct SearchResultsPage {
    results: Vec<DecodedResult>,
}

impl SearchResultsPage {
    pub fn new(parsed: &Vec<SearchResult>) -> Result<SearchResultsPage, Google2005Error> {
        let mut results: Vec<DecodedResult> = vec![];

        for result in parsed {
            let decoded_url = decode(result.url()).unwrap();
            let joined_title = result.title().join(" ");
            results.push(DecodedResult {
                url: decoded_url.to_string(),
                title: joined_title.to_string(),
            });
        }

        Ok(SearchResultsPage { results: results })
    }
}
