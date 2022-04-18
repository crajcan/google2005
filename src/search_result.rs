use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchResult<'a> {
    pub url: &'a str,
    title: Vec<&'a str>,
    pub description: Vec<&'a str>,
}

impl<'a> SearchResult<'a> {
    pub fn new(url: &'a str, title: Vec<&'a str>) -> Self {
        SearchResult {
            url,
            title,
            description: vec![],
        }
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn title(&self) -> &Vec<&str> {
        &self.title
    }

    pub fn is_junk(&self) -> bool {
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

    pub fn web_page(&self) -> &str {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
