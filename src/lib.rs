use reqwest::Client;

//use reqwest to google for the query
pub async fn search_for_web_results(query: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://www.google.com/search?q={}", query);
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google() {
        println!("res: {:?}", search_for_web_results("blough").await.unwrap());
    }
}
