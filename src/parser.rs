use std::ops::Deref;

use crate::search_result::SearchResult;
use ego_tree::NodeRef;
use scraper::{ElementRef, Html, Node, Selector};

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
                println!("element: {:?}", element);
                let title = element.attr("text").unwrap();

                search_results.push(SearchResult::new(url, vec![title]));
            } else if element.name() == "span" {
                let description = element.attr("text").unwrap();


                search_results.last_mut().unwrap().description = vec![description];
            } else {
                for child in e.children() {
                    walk(&child, search_results);
                }
            }
        }
        _ => {}
    }
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
}
