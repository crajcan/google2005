use reqwest::Client;

mod parser;
use parser::parse;
mod filter;
use askama::Template;
use filter::{filtered_links, SearchResult};
use scraper::Html;
use serde::Serialize;
use std::fmt::Debug;
use urlencoding::decode;

#[derive(Debug, Serialize)]
struct DecodedResult {
    url: String,
    title: String,
}

#[derive(Debug, Serialize, Template)]
#[template(path = "search.html")]
pub struct SearchResultsPage {
    name: String,
    results: Vec<DecodedResult>,
}

#[derive(Debug, Serialize)]
pub struct MyError {
    report: String,
}

pub async fn google(query: &str) -> Result<String, MyError> {
    match search_for_web_results(query).await {
        Ok(results) => {
            let dom = Html::parse_document(&results);
            let mut links = parse(&dom);
            let filtered_links = filtered_links(&mut links);
            let page = build(filtered_links).unwrap();
            let html = page.render().unwrap();
            Ok(html)
        }
        Err(e) => {
            println!("error: {}", e);
            Err(MyError {
                report: e.to_string(),
            })
        }
    }
}

//use reqwest to google for the query
pub async fn search_for_web_results(query: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    Ok(body)
}

fn build<'a>(parsed: &Vec<SearchResult>) -> Result<SearchResultsPage, MyError> {
    let mut results: Vec<DecodedResult> = vec![];

    for result in parsed {
        let decoded_url = decode(result.url()).unwrap();
        let joined_title = result.title().join(" ");
        results.push(DecodedResult {
            url: decoded_url.to_string(),
            title: joined_title.to_string(),
        });
    }

    Ok(SearchResultsPage {
        name: "Carson".to_string(),
        results: results,
    })
}