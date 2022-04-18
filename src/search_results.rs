use crate::parser::parse;
use crate::search_result::SearchResult;
use scraper::Html;
use std::ops::Deref;

pub struct SearchResults<'a> {
    pub results: Vec<SearchResult<'a>>,
}

impl<'a> Deref for SearchResults<'a> {
    type Target = Vec<SearchResult<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.results
    }
}

impl<'a> SearchResults<'a> {
    pub fn new(dom: &'a Html) -> Self {
        SearchResults {
            results: parse(&dom),
        }
    }

    pub fn filter(&mut self) -> &mut Self {
        self.remove_junk();
        self.strip_analytics_bs();
        self.remove_redundant_pages();

        if self.len() != 10 {
            println!("Error: Should return 10 links, {} links found", self.len());

            println!("{:#?}", self.results);
        }

        self
    }

    fn remove_junk(&mut self) {
        self.results
            .retain(|search_result| !search_result.is_junk());
    }

    fn strip_analytics_bs(&mut self) {
        for result in self.results.iter_mut() {
            result.url = between("url?q=", "&sa=U", result.url)
        }
    }

    fn remove_redundant_pages(&mut self) {
        let mut unique_pages: Vec<String> = vec![];

        self.results.retain(|result| {
            if unique_pages.contains(&result.web_page().to_string()) {
                false
            } else {
                unique_pages.push(result.web_page().to_string());
                true
            }
        });
    }
}

fn between<'a, 'b>(start: &'a str, end: &'a str, s: &'b str) -> &'b str {
    let start_index = s.find(start).unwrap_or(0);
    let end_index = s.find(end).unwrap_or(s.len());
    let between = &s[start_index + start.len()..end_index];
    between
}

#[allow(dead_code)]
fn bookended_with(start: &str, end: &str, s: &str) -> String {
    let start_index = s.find(start).unwrap() - start.len();
    let end_index = s.find(end).unwrap() + end.len();
    let between = &s[start_index + start.len()..end_index];
    between.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_redundant_pages() {
        let mut search_results = SearchResults {
            results: vec![
                SearchResult::new(
                    "https://en.wikipedia.org/wiki/David_Blough#2015_season",
                ),
                SearchResult::new(
                    "https://en.wikipedia.org/wiki/David_Blough#College_Career",
                ),
                SearchResult::new(
                    "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
                ),
                SearchResult::new(
                    "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=2347815098",
                ),
            ],
        };

        search_results.remove_redundant_pages();

        assert_eq!(
            search_results
                .results
                .iter()
                .map(|l| l.url)
                .collect::<Vec<&str>>(),
            vec![
                "https://en.wikipedia.org/wiki/David_Blough#2015_season",
                "https://www.lowes.com/pl/Cordless--Drills/4294607722?refinement=4294776932",
            ]
        )
    }
}
