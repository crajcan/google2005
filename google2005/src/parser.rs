#![allow(dead_code)]

use std::ops::Deref;

use crate::search_result::SearchResult;
use ego_tree::NodeRef;
use scraper::{ElementRef, Html, Node, Selector};

const HEADINGS: &'static [&'static str] = &["h1", "h2", "h3", "h4", "h5", "h6"];

pub fn parse(dom: &Html) -> Vec<SearchResult> {
    let body = dom
        .select(&Selector::parse("body").unwrap())
        .next()
        .unwrap();

    let mut search_results = vec![];
    let node_ref = Deref::deref(&body);

    walk(node_ref, &mut search_results);

    search_results
}

fn walk<'a>(e: &NodeRef<'a, Node>, search_results: &mut Vec<SearchResult<'a>>) {
    let v = e.value();

    match v {
        Node::Element(element) => {
            if element.name() == "a" {
                //create a search result for the elem and try to add a title

                let url = match element.attr("href") {
                    Some(path) => path,
                    None => ""
                };

                let title = copy_from_headings(e);
                let mut search_result = SearchResult::new(url);
                search_result.title = Some(title);

                search_results.push(search_result);
            } else if element.name() == "span" {
                if search_results.len() != 0
                {
                    let description = all_copy(e);

                    if description.contains(&"People also ask") {
                        // pushing another (dummy) search result will avoid adding the
                        // "people also ask" nonsense to our valid search results
                        search_results.push(SearchResult::new(""));
                    }

                    //add to description
                    search_results.last_mut().unwrap().add_to_description(description); }
            } else if element.name() == "script" {
                return
            } else {
                for child in e.children() {
                    walk(&child, search_results);
                }
            }
        }
        Node::Text(description) => {
            //add a decription to the last search result if there is none
            if search_results.len() != 0 && description.starts_with("http")
            {
                search_results.last_mut().unwrap().add_to_description(vec![&(**description)]);
            }
        }
        _ => {}
    }
}

fn copy_from_headings<'a>(e: &NodeRef<'a, Node>) -> Vec<&'a str> {
    let mut copy = vec![];

    let v = e.value();

    match v {
        Node::Element(element) => {
            let name = element.name();

            if HEADINGS.contains(&name) {
                return all_copy(e);
            } else {
                for child in e.children() {
                    copy.append(&mut copy_from_headings(&child));
                }
            }
        }
        _ => {}
    }

    copy
}

//gets all text from an html element and it's offspring
fn all_copy<'a>(e: &NodeRef<'a, Node>) -> Vec<&'a str> {
    let mut copy = vec![];

    let v = e.value();

    match v {
        Node::Element(_element) => {
            for child in e.children() {
                copy.append(&mut all_copy(&child));
            }
        }
        Node::Text(text) => {
            copy.push(&(**text));
        }
        _ => {}
    }

    copy
}

fn get_text(element: ElementRef) -> Vec<&str> {
    element.text().collect::<Vec<_>>()
}

fn get_href(element: ElementRef) -> &str {
    element.value().attr("href").unwrap()
}

fn get_elems<'a, 'b>(fragment: &'a Html, selector: &'b str) -> Vec<ElementRef<'a>> {
    let selector = Selector::parse(selector).unwrap();

    let mut result = vec![];
    for element in fragment.select(&selector) {
        result.push(element);
    }

    result
}

fn links(fragment: &Html) -> Vec<ElementRef> {
    get_elems(fragment, "a")
}

#[allow(dead_code)]
fn h1s(fragment: &Html) -> Vec<ElementRef> {
    get_elems(fragment, "h1")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_walk_scrapes_description() {
        let section = concat!(
            "<body>",
            "<div>",
            r#"<div class="ZINbbc luh4tb xpd O9g5cc uUPGi">"#,
            r#"<div class="egMi0 kCrYT">"#,
            r#"<a href="/url?q=https://www.foxsports.com/nfl/david-blough">"#,
            r#"<h3 class="zBAuLc l97dzf">"#,
            r#"<div class="BNeawe vvjwJb AP7Wnd">David Blough - NFL News, Rumors, &amp; Updates - FOX Sports</div>"#,
            "</h3>",
            r#"<div class="BNeawe UPmit AP7Wnd">www.foxsports.com &#8250; nfl &#8250; david-blough-player</div>"#,
            "</a>",
            "</div>",
            r#"<div class="kCrYT"><div>"#,
            r#"<div class="BNeawe s3v9rd AP7Wnd">"#,
            "<div>",
            "<div>",
            r#"<div class="BNeawe s3v9rd AP7Wnd">Remains No. 3 QB Blough (coach's decision) is inactive for Thursday's game against the Bears. Impact While dressing as the No.</div>"#,
            "</div>",
            "</div>",
            "</div>",
            "</div>",
            "</div>",
            "</div>",
            "</div>",
            "</body>",
        );

        let body = Html::parse_document(section);
        let elem = body 
            .select(&Selector::parse("body").unwrap())
            .next()
            .unwrap();
        let node_ref = Deref::deref(&elem);

        let mut search_results = vec![];
        walk(node_ref, &mut search_results);

        assert_eq!(
            search_results[0],
            SearchResult { 
                url: "/url?q=https://www.foxsports.com/nfl/david-blough",
                title: Some(vec![
                    "David Blough - NFL News, Rumors, & Updates - FOX Sports",
                ]),
                description: Some(vec![
                    "Remains No. 3 QB Blough (coach's decision) is inactive for Thursday's game against the Bears. Impact While dressing as the No.",
                ]),
            }
        );       
    }

    #[test]
    fn test_walk_scrapes_description_with_time() {
        let section = concat!(
            r#"<div>"#,
            r#"<div class="ZINbbc luh4tb xpd O9g5cc uUPGi">"#,
            r#"<div class="egMi0 kCrYT">"#,
            r#"<a href="/url?q=https://www.cbssports.com/mlb/teams/">"#,
            r#"<h3 class="zBAuLc l97dzf">"#,
            r#"<div class="BNeawe">Chicago Cubs News, Schedule - MLB - CBS Sports</div>"#,
            r#"</h3>"#,
            r#"<div class="BNeawe UPmit AP7Wnd">www.cbssports.com &#8250; mlb &#8250; teams &#8250; CHC &#8250; chicago-cubs</div>"#,
            r#"</a>"#,
            r#"</div>"#,
            r#"<div class="kCrYT">"#,
            r#"<div>"#,
            r#"<div class="BNeawe s3v9rd AP7Wnd">"#,
            r#"<div>"#,
            r#"<div>"#,
            r#"<div class="BNeawe s3v9rd AP7Wnd">"#,
            r#"<span class="xUrNXd UMOHqf">21 hours ago</span>"#,
            r#"<span class="xUrNXd UMOHqf"> · </span>"#,
            r#"Get the latest news and information for the Chicago Cubs. 2022 season schedule, scores, stats, and highlights. Find out the latest on your favorite MLB ..."#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
            r#"</div>"#,
        );

        let body = Html::parse_document(section);
        let elem = body 
            .select(&Selector::parse("body").unwrap())
            .next()
            .unwrap();
        let node_ref = Deref::deref(&elem);

        let mut search_results = vec![];
        walk(node_ref, &mut search_results);

        assert_eq!(
            search_results[0],
            SearchResult { 
                url: "/url?q=https://www.cbssports.com/mlb/teams/",
                title: Some(vec![
                    "Chicago Cubs News, Schedule - MLB - CBS Sports",
                ]),
                description: Some(vec![
                    "21 hours ago",
                    " · ",
                    "Get the latest news and information for the Chicago Cubs. 2022 season schedule, scores, stats, and highlights. Find out the latest on your favorite MLB ...",
                ]),
            }
        );       
    }


    #[test]
    fn test_get_text() {
        let html = "<h1>Hello World</h1>";
        let dom = Html::parse_document(html);
        let selector = Selector::parse("h1").unwrap();
        let h1 = dom.select(&selector).next().unwrap();

        assert_eq!(get_text(h1), vec!["Hello World"]);
    }

    #[test]
    fn test_get_href() {
        let html = "<a href=\"https://wikipedia.org\">Hello World</a>";
        let fragment = Html::parse_document(html);
        let selector = Selector::parse("a").unwrap();
        let a = fragment.select(&selector).next().unwrap();

        assert_eq!(get_href(a), "https://wikipedia.org");
    }

    #[test]
    fn test_get_elems() {
        let html = r#"
            <ul>
                <li>Foo</li>
                <li>Bar</li>
                <li>Baz</li>
            </ul>
        "#;

        let fragment = Html::parse_document(html);

        assert_eq!(
            get_elems(&fragment, "li")
                .iter()
                .map(|e| get_text(*e))
                .collect::<Vec<_>>(),
            vec![vec!["Foo"], vec!["Bar"], vec!["Baz"]]
        );
    }

    #[test]
    fn test_links() {
        let html = r#"
            <ul>
                <li><a href="foo">Foo</a></li>
                <li><a href="bar">Bar</a></li>
                <li><a href="baz">Baz</a></li>
            </ul>
        "#;

        let fragment = Html::parse_document(html);

        assert_eq!(
            links(&fragment)
                .iter()
                .map(|e| get_text(*e))
                .collect::<Vec<_>>(),
            vec![vec!["Foo"], vec!["Bar"], vec!["Baz"]]
        );
    }

    // #[test]
    #[allow(dead_code)]
    fn test_h1s() {
        let html = r#"
            <html>
                <head>
                    <title>Hello World</title>
                </head>
                <body>
                    <h1>Hello World</h1>
                </body>
            </html>
        "#;

        let fragment = Html::parse_document(html);

        assert_eq!(
            h1s(&fragment)
                .iter()
                .map(|h1| get_text(*h1))
                .collect::<Vec<_>>(),
            vec![vec!["Hello World"]]
        );
    }

    #[test]
    fn test_copy_from_headings() {
        let anchor = concat!(
            "<a>",
            r#"<h3 class="LC20lb MBeuO DKV0Md">"#,
            "David Blough Stats, News and Video - QB | NFL.com",
            "</h3>",
            r#"<div class="TbwUpd NJjxre">"#,
            r#"<cite class="iUh30 qLRx3b tjvcx" role="text">"#,
            "https://www.nfl.com",
            r#"<span class="dyjrff qzEoUe" role="text"> › players › david-blough</span>"#,
            "</cite>",
            "</div>",
            "</a>"
        );

        let dom = Html::parse_document(anchor);
        let selector = Selector::parse("a").unwrap();
        let a = dom.select(&selector).next().unwrap();
        let node_ref = Deref::deref(&a);

        assert_eq!(
            copy_from_headings(node_ref),
            vec!["David Blough Stats, News and Video - QB | NFL.com"]
        );
    }

    #[test]
    fn test_copy_from_headings_with_interior_div() {
        let anchor = concat!(
            r#"<a href="/url?q=https://www.detroitlions.com/team/players-roster/david-blough/">"#,
            r#"<h3 class="zBAuLc l97dzf">"#,
            r#"<div class="BNeawe vvjwJb AP7Wnd">David Blough - Detroit Lions</div>"#,
            "</h3>",
            r#"<div class="BNeawe UPmit AP7Wnd">www.detroitlions.com &#8250; team &#8250; players-roster &#8250; david-blough</div>"#,
            "</a>"
        );

        let dom = Html::parse_document(anchor);
        let selector = Selector::parse("a").unwrap();
        let a = dom.select(&selector).next().unwrap();
        let node_ref = Deref::deref(&a);

        assert_eq!(
            copy_from_headings(node_ref),
            vec!["David Blough - Detroit Lions"]
        );
    }

    #[test]
    fn test_all_copy() {
        let span = concat!(
            "<span>Latest on QB David ",
            "<em>Blough</em>",
            " including news, stats, videos, highlights and more on NFL.com.",
            "</span>"
        );

        let dom = Html::parse_document(span);
        let selector = Selector::parse("span").unwrap();
        let span = dom.select(&selector).next().unwrap();
        let node_ref = Deref::deref(&span);

        assert_eq!(
            all_copy(node_ref),
            vec![
                "Latest on QB David ",
                "Blough",
                " including news, stats, videos, highlights and more on NFL.com."
            ]
        );
    }
}
