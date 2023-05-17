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
    page_start: u16,
    page: u16,
    image_hostname: String,
    stylesheet_hostname: String,
}

impl SearchResultsResponse {
    pub fn new(
        parsed: &SearchResults,
        query: SearchRequest,
    ) -> Result<SearchResultsResponse, Google2005Error> {
        let mut results: Vec<DecodedResult> = vec![];

        if parsed.results.len() == 0 {
            return Err(Google2005Error::new(None, Some("No results found")));
        }

        for result in &parsed.results {
            // println!("*** result.url: {}", result.url);
            let decoded_url = decode(result.url).unwrap();
            // println!("** decoded_url: {}", decoded_url);
            let joined_title = result.title.as_ref().unwrap().join(" ");
            let description = result.joined_and_decoded_description();

            results.push(DecodedResult {
                url: decoded_url.to_string(),
                title: joined_title.to_string(),
                description: description,
            });
        }

        Ok(SearchResultsResponse {
            results: results,
            query: query.search_string,
            first_result: Self::response_start(query.start),
            next_page_starts: Self::next_page_starts(query.start),
            last_result: Self::response_start(query.start)
                + parsed.results.len() as u16,
            page_start: Self::page_start(query.start),
            page: Self::page(query.start),
            image_hostname: Self::image_hostname(),
            stylesheet_hostname: Self::stylesheet_hostname(),
        })
    }

    fn response_start(requested_start: u16) -> u16 {
        match requested_start / 10 {
            0 => 1,
            x => x * 10,
        }
    }

    fn page_start(requested_start: u16) -> u16 {
        match requested_start / 10 {
            0 => 0,
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

    fn page(start: u16) -> u16 {
        (start / 10) + 1
    }

    fn image_hostname() -> String {
        "https://google2005.s3.us-east-2.amazonaws.com/images/".to_string()
    }

    fn stylesheet_hostname() -> String {
        "https://google2005.s3.us-east-2.amazonaws.com/stylesheets/".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_page_handles_zero() {
        let start = 0;
        assert_eq!(SearchResultsResponse::page(start), 1);
    }

    #[test]
    fn test_page_handles_one() {
        let start = 1;
        assert_eq!(SearchResultsResponse::page(start), 1);
    }

    #[test]
    fn test_page_handles_ten() {
        let start = 10;
        assert_eq!(SearchResultsResponse::page(start), 2);
    }

    #[test]
    fn test_page_handles_eleven() {
        let start = 11;
        assert_eq!(SearchResultsResponse::page(start), 2);
    }

    #[test]
    fn test_page_handles_twenty() {
        let start = 20;
        assert_eq!(SearchResultsResponse::page(start), 3);
    }

    #[test]
    fn test_page_handles_twenty_one() {
        let start = 21;
        assert_eq!(SearchResultsResponse::page(start), 3);
    }

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
