use reqwest::Client;

mod parser;
use parser::parse;
mod filter;
use filter::filtered_links;
use scraper::Html;
use std::fmt::Debug;
use urlencoding::decode;

#[derive(Debug)]
pub struct MyError {
    report: String,
}

pub async fn google(query: &str) -> Result<String, MyError> {
    match search_for_web_results(query).await {
        Ok(results) => {
            let dom = Html::parse_document(&results);
            let mut links = parse(&dom);
            let filtered_links = filtered_links(&mut links);
            build(filtered_links)
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

fn build(parsed: &Vec<(&str, Vec<&str>)>) -> Result<String, MyError> {
    let mut results = "".to_string();
    results.push_str("<!DOCTYPE html>");
    results.push_str(r#"<html lang="en">"#);

    for (i, line) in parsed.iter().enumerate() {
        let url = decode(line.0).unwrap();

        results.push_str(&format!(
            r#"<p><em>{}. </em><a href={}>{}</a></br>"#,
            i + 1,
            url,
            line.1.join(" ")
        ));
        results.push_str(&format!(r#"<span>{}</span></p>"#, url));
    }
    results.push_str("</html>");

    Ok(results)
}
