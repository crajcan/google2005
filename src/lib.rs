use askama::Template;
use reqwest::Client;
use scraper::Html;

mod filter;
mod parser;
mod search_result;
mod search_results_page;
mod utils;

use filter::filtered_links;
use parser::parse;
use search_results_page::SearchResultsPage;
use utils::google2005_error::Google2005Error;

pub async fn google(query: &str) -> Result<String, Google2005Error> {
    match search_for_web_results(query).await {
        Ok(results) => {
            let dom = Html::parse_document(&results);

            let mut anchors = parse(&dom);

            let search_results = filtered_links(&mut anchors);

            let page = SearchResultsPage::new(search_results).unwrap();

            let html = page.render().unwrap();

            Ok(html)
        }
        Err(e) => Err(Google2005Error::new(e.to_string())),
    }
}

//use reqwest to google for the query
async fn search_for_web_results(query: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    Ok(body)
}
