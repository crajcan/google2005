use askama::Template;
use reqwest::Client;
use scraper::Html;

mod parser;
mod search_result;
mod search_results;
mod search_results_page;
mod utils;

use search_results::SearchResults;
use search_results_page::SearchResultsPage;
use utils::google2005_error::Google2005Error;

pub async fn google(query: &str) -> Result<String, Google2005Error> {
    let results = request_search_from_google(query).await?;
    let dom = Html::parse_document(&results);

    let mut hyperlinks = SearchResults::new(&dom);
    let search_results = hyperlinks.filter();

    let page = SearchResultsPage::new(&search_results)?.render()?;

    Ok(page)
}

//use reqwest to google for the query
async fn request_search_from_google(query: &str) -> Result<String, Google2005Error> {
    let client = Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
