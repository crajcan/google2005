//use filter to filter out links that contain "search?q=""
pub fn filtered_links<'a>(
    links: &'a mut Vec<(&'a str, Vec<&'a str>)>,
) -> &'a Vec<(&'a str, Vec<&'a str>)> {
    links.retain(|(link, description)| !is_junk(link, description));

    links
}

fn is_junk(link: &str, description: &Vec<&str>) -> bool {
    is_alternative_search(link)
        || is_image_link(link)
        || is_google_logistics(description)
        || is_google_logo(description)
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