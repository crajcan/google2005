use askama::Template;
use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct SearchResult<'a> {
        url: &'a str,
        title: Vec<&'a str>,
    }

//use filter to filter out links that contain "search?q=""
pub fn filtered_links<'a>(results: &'a mut Vec<SearchResult<'a>>) -> &'a mut Vec<SearchResult<'a>> {
    remove_junk(results);
    strip_analytics_bs(results);
    remove_redundant_pages(results);

    if results.len() != 10 {
        println!(
            "Error: Should return 10 links, {} links found",
            results.len()
        );

        println!("{:#?}", results);
    }

    results
}

fn remove_junk(results: &mut Vec<SearchResult>) {
    results.retain(|search_result| !search_result.is_junk());
}

fn strip_analytics_bs(results: &mut Vec<SearchResult>) {
    for result in results.iter_mut() {
        result.url = between("url?q=", "&sa=U", result.url)
    }
}

fn remove_redundant_pages(results: &mut Vec<SearchResult>) {
    let mut unique_pages: Vec<String> = vec![];

    results.retain(|result| {
        if unique_pages.contains(&result.web_page().to_string()) {
            false
        } else {
            unique_pages.push(result.web_page().to_string());
            true
        }
    });
}

impl<'a> SearchResult<'a> {
    pub fn new(url: &'a str, title: Vec<&'a str>) -> Self {
        SearchResult { url, title }
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn title(&self) -> &Vec<&str> {
        &self.title
    }

    fn is_junk(&self) -> bool {
        self.is_alternative_search()
            || self.is_google_logo()
            || self.is_image_link()
            || self.is_google_ad()
            || self.is_google_logistics()
    }

    fn is_alternative_search(&self) -> bool {
        self.url.contains("search?q=")
            || self.url.contains("search?ie=")
            || self.url.contains("&ie=")
    }

    fn is_google_logo(&self) -> bool {
        *self.title == vec!["G", "o", "o", "g", "l", "e"]
    }

    fn is_image_link(&self) -> bool {
        self.url.contains("imgres?imgurl=")
    }

    fn is_google_ad(&self) -> bool {
        self.url.starts_with("http://www.google.com/aclk?")
            || self.url.starts_with("https://www.google.com/aclk?")
    }

    fn is_google_logistics(&self) -> bool {
        (self.title.join(" ") == "Privacy")
            || (self.title.join(" ") == "Learn more")
            || (self.title.join(" ") == "Settings")
            || (self.title.join(" ") == "Terms")
            || (self.title.join(" ") == "Sign in")
            || (self.title.join(" ") == "Search tools")
    }

    fn web_page(&self) -> &str {
        let segments = self
            .url
            .split(&['%', '?', '&', '#'][..])
            .collect::<Vec<&str>>();

        if segments.len() > 1 {
            segments[0]
        } else {
            self.url
        }
    }
}

fn between<'a, 'b>(start: &'a str, end: &'a str, s: &'b str) -> &'b str {
    let start_index = s.find(start).unwrap_or(0);
    let end_index = s.find(end).unwrap_or(s.len());
    let between = &s[start_index + start.len()..end_index];
    between
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

    #[test]
    fn test_web_page_returns_segment_before_question_mark() {
        let result = SearchResult::new(
            "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
            vec!["Cordless", "Drills"],
        );

        assert_eq!(
            result.web_page(),
            "https://www.lowes.com/pl/Cordless--Drills/4294607722",
        )
    }

    #[test]
    fn test_web_page_returns_segment_before_pound_sign() {
        let result = SearchResult::new(
            "https://en.wikipedia.org/wiki/David_Blough#2015_season",
            vec!["David", "Blough"],
        );

        assert_eq!(
            result.web_page(),
            "https://en.wikipedia.org/wiki/David_Blough",
        )
    }

    #[test]
    fn test_web_page_returns_segment_before_percent_sign() {
        let result = SearchResult::new(
            "https://en.wikipedia.org/wiki/David_Blough%232015_season",
            vec!["David", "Blough"],
        );

        assert_eq!(
            result.web_page(),
            "https://en.wikipedia.org/wiki/David_Blough",
        )
    }

    #[test]
    fn test_redundant_pages() {
        let mut links = vec![
            SearchResult::new(
                "https://en.wikipedia.org/wiki/David_Blough#2015_season",
                vec![],
            ),
            SearchResult::new(
                "https://en.wikipedia.org/wiki/David_Blough#College_Career",
                vec![],
            ),
            SearchResult::new(
                "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
                vec![],
            ),
            SearchResult::new(
                "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=2347815098",
                vec![],
            ),
        ];

        remove_redundant_pages(&mut links);

        assert_eq!(
            links.iter().map(|l| l.url).collect::<Vec<&str>>(),
            vec![
                "https://en.wikipedia.org/wiki/David_Blough#2015_season",
                "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
            ]
        )
    }
}
