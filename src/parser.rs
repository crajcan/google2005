use std::ops::Deref;

use crate::search_result::SearchResult;
use ego_tree::NodeRef;
use scraper::node::Element;
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
                let url = element.attr("href").unwrap();
                let title = copy_from_headings(e);

                search_results.push(SearchResult::new(url));
            } else {
                for child in e.children() {
                    walk(&child, search_results);
                }
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
                for child in e.children() {
                    if let Node::Text(text) = child.value() {
                        copy.push(&(**text));
                    }
                }
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
    fn test_str_in() {
        let headings = &["h1", "h2", "h3", "h4", "h5", "h6"];

        assert!(str_in(headings, "h3"));
        assert!(str_in(headings, "h6"));
        assert!(!str_in(headings, "h7"));
    }
}

fn str_in(list: &[&str], s: &str) -> bool {
    list.iter().any(|x| *x == s)
}
