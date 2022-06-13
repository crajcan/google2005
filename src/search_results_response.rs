use askama::Template;
use serde::Serialize;
use urlencoding::decode;

use crate::{
    search_request::SearchRequest, search_results::SearchResults,
    utils::google2005_error::Google2005Error,
};

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
    first_result: u16,
    next_page_starts: Vec<u16>,
    last_result: u16,
}

impl SearchResultsResponse {
    pub fn new(
        parsed: &SearchResults,
        query: SearchRequest,
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
            query: query.search_string,
            first_result: Self::response_start(query.start),
            last_result: Self::response_start(query.start) + parsed.results.len() as u16,
            next_page_starts: Self::next_page_starts(query.start),
        })
    }

    fn response_start(requested_start: u16) -> u16 {
        match requested_start / 10 {
            0 => 1,
            x => x * 10,
        }
    }

    fn next_page_starts(start: u16) -> Vec<u16> {
        if start < 60 {
            (0..10_u16).map(|x| x * 10).collect::<Vec<u16>>()
        } else {
            let mut starts = vec![];

            let first = start - 60;

            for i in 1..=10 {
                starts.push(first + i * 10);
            }

            starts
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_page_starts() {
        let start = 0;
        assert_eq!(
            SearchResultsResponse::next_page_starts(start),
            vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90],
        );

        let start = 40;
        assert_eq!(
            SearchResultsResponse::next_page_starts(start),
            vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90],
        );

        let start = 50;
        assert_eq!(
            SearchResultsResponse::next_page_starts(start),
            vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90],
        );

        let start = 60;
        assert_eq!(
            SearchResultsResponse::next_page_starts(start),
            vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        );

        let start = 100;
        assert_eq!(
            SearchResultsResponse::next_page_starts(start),
            vec![50, 60, 70, 80, 90, 100, 110, 120, 130, 140],
        );
    }
}
