//use filter to filter out links that contain "search?q=""
pub fn filtered_links<'a>(
    links: &'a mut Vec<(&'a str, Vec<&'a str>)>,
) -> &'a mut Vec<(&'a str, Vec<&'a str>)> {
    remove_junk(links);
    strip_analytics_bs(links);
    // remove_redundant_pages(links);

    links
}

fn remove_junk(links: &mut Vec<(&str, Vec<&str>)>) {
    links.retain(|(link, description)| !is_junk(link, description));
}

fn is_junk(link: &str, description: &Vec<&str>) -> bool {
    is_alternative_search(link)
        || is_google_logo(description)
        || is_image_link(link)
        || is_google_ad(link)
        || is_google_logistics(description)
}

fn is_alternative_search(link: &str) -> bool {
    link.contains("search?q=") || link.contains("search?ie=") || link.contains("&ie=")
}

fn is_image_link(link: &str) -> bool {
    link.contains("imgres?imgurl=")
}

fn is_google_logistics(description: &Vec<&str>) -> bool {
    (description.join(" ") == "Privacy")
        || (description.join(" ") == "Learn more")
        || (description.join(" ") == "Settings")
        || (description.join(" ") == "Terms")
        || (description.join(" ") == "Sign in")
        || (description.join(" ") == "Search tools")
}

fn is_google_logo(description: &Vec<&str>) -> bool {
    *description == vec!["G", "o", "o", "g", "l", "e"]
}

fn is_google_ad(link: &str) -> bool {
    link.starts_with("http://www.google.com/aclk?")
        || link.starts_with("https://www.google.com/aclk?")
}

fn strip_analytics_bs(links: &mut Vec<(&str, Vec<&str>)>) {
    for (link, description) in links.iter_mut() {
        *link = between("url?q=", "&sa=U", link)
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

// fn remove_redundant_pages(links: &mut Vec<(&str, Vec<&str>)>) {
//     let mut unique_pages = vec![];

//     for (i, (link, description)) in links.iter().enumerate() {
//         if unique_pages.includes(link.split(
//     }

// }

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

    // #[test]
    // fn test_web_page_returns_segment_before_percent() {
    //     let input =
    // }

    // #[test]
    // fn test_web_page_returns_segment_before_ampersand() {
    // }
}
