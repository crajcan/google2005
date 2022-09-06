use serde::Serialize;
use urlencoding::decode;

#[derive(Debug, Serialize, PartialEq)]
pub struct SearchResult<'a> {
    pub url: &'a str,
    pub title: Option<Vec<&'a str>>,
    pub description: Option<Vec<&'a str>>,
}

impl<'a> SearchResult<'a> {
    pub fn new(url: &'a str) -> Self {
        SearchResult {
            url,
            title: None,
            description: None,
        }
    }

    pub fn add_to_description(&mut self, description: Vec<&'a str>) {
        if self.description.is_none() {
            self.description = Some(description);
        } else {
            self.description.as_mut().unwrap().extend(description);
        }
    }

    pub fn is_regular_result(&self) -> bool {
        !self.is_alternative_search()
            && !self.is_google_logo()
            && !self.is_image_link()
            && !self.is_google_ad()
            && !self.is_google_logistics()
            && self.has_title_and_description()
    }

    fn is_alternative_search(&self) -> bool {
        self.url.contains("search?q=")
            || self.url.contains("search?ie=")
            || self.url.contains("&ie=")
    }

    fn has_title_and_description(&self) -> bool {
        self.title.is_some()
            && !self.title.as_ref().unwrap().is_empty()
            && self.description.is_some()
            && !self.description.as_ref().unwrap().is_empty()
    }

    fn is_google_logo(&self) -> bool {
        self.title == Some(vec!["G", "o", "o", "g", "l", "e"])
    }

    fn is_image_link(&self) -> bool {
        self.url.contains("imgres?imgurl=")
    }

    fn is_google_ad(&self) -> bool {
        self.url.starts_with("http://www.google.com/aclk?")
            || self.url.starts_with("https://www.google.com/aclk?")
    }

    fn is_google_logistics(&self) -> bool {
        match &self.title {
            Some(title) => {
                (title.join(" ") == "Privacy")
                    || (title.join(" ") == "Learn more")
                    || (title.join(" ") == "Settings")
                    || (title.join(" ") == "Terms")
                    || (title.join(" ") == "Sign in")
                    || (title.join(" ") == "Search tools")
            }
            None => false,
        }
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

    pub fn joined_and_decoded_description(&self) -> String {
        println!("this search result description: {:#?}", self.description);

        let joined_description = match self.description.as_ref() {
            Some(description) => description.join(" "),
            None => String::from(""),
        };

        let res = decode(&joined_description).unwrap().to_string();
        println!("this rectified result: {:#?}\n", res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_page_returns_segment_before_question_mark() {
        let mut result = SearchResult::new(
            "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
        );
        result.title = Some(vec!["Cordless", "Drills"]);

        assert_eq!(
            result.web_page(),
            "https://www.lowes.com/pl/Cordless--Drills/4294607722",
        )
    }

    #[test]
    fn test_web_page_returns_segment_before_pound_sign() {
        let mut result =
            SearchResult::new("https://en.wikipedia.org/wiki/David_Blough#2015_season");
        result.title = Some(vec!["David", "Blough"]);

        assert_eq!(
            result.web_page(),
            "https://en.wikipedia.org/wiki/David_Blough",
        )
    }

    #[test]
    fn test_web_page_returns_segment_before_percent_sign() {
        let mut result =
            SearchResult::new("https://en.wikipedia.org/wiki/David_Blough%232015_season");
        result.title = Some(vec!["David", "Blough"]);

        assert_eq!(
            result.web_page(),
            "https://en.wikipedia.org/wiki/David_Blough",
        )
    }

    #[test]
    fn test_add_to_description() {
        let mut result = SearchResult::new("https://www.lowes.com/");

        result.add_to_description(vec!["Cordless Drills", "Miter Saws"]);
        result.add_to_description(vec!["Screw Drivers"]);

        assert_eq!(
            result.description.unwrap(),
            vec!["Cordless Drills", "Miter Saws", "Screw Drivers"]
        )
    }
}
