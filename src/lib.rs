use reqwest::Client;

mod parser;
use parser::parse;
use scraper::Html;
use std::fmt::Debug;

#[derive(Debug)]
pub struct MyError {
    report: String,
}

pub async fn google(query: &str) -> Result<String, MyError> {
    match search_for_web_results(query).await {
        Ok(results) => {
            let dom = Html::parse_document(&results);
            let links = parse(&dom);
            let filtered_links = filtered_links(links);
            build(links)
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
    let client = reqwest::Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    Ok(body)
}

fn build(parsed: impl Debug) -> Result<String, MyError> {
    let results = format!("parsed: {:#?}", parsed);

    Ok(results)
}

fn between(start: &str, end: &str, s: &str) -> String {
    let start_index = s.find(start).unwrap();
    let end_index = s.find(end).unwrap();
    let between = &s[start_index + start.len()..end_index];
    between.to_string()
}

fn bookended_with(start: &str, end: &str, s: &str) -> String {
    let start_index = s.find(start).unwrap() - start.len();
    let end_index = s.find(end).unwrap() + end.len();
    let between = &s[start_index + start.len()..end_index];
    between.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    #[test]
    fn test_between() {
        let input = "http://localhost:7878/url?q=https://www.sbnation.com/authors/jon-bois&sa=U&ved=2ahUKEwj91bWk2IT3AhV1omoFHTGiCBgQFnoECAwQAg&usg=AOvVaw0tLu83JeMGMgnFF9iLD2uA";

        assert_eq!(
            between("url?q=", "&sa=U", input),
            "https://www.sbnation.com/authors/jon-bois"
        )
    }

    #[test]
    fn test_bookended_with() {
        let input = "fooo<a>bar</a>baz";

        assert_eq!(bookended_with("<a>", "</a>", input), "<a>bar</a>")
    }
}
