use askama::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct HomePageResponse {
    image_hostname: String,
    stylesheet_hostname: String,
}

impl HomePageResponse {
    pub fn new() -> HomePageResponse {
        HomePageResponse {
            image_hostname: Self::image_hostname(),
            stylesheet_hostname: Self::stylesheet_hostname(),
        }
    }

    fn image_hostname() -> String {
        "https://google2005.s3.us-east-2.amazonaws.com/images/".to_string()
    }

    fn stylesheet_hostname() -> String {
        "https://google2005.s3.us-east-2.amazonaws.com/stylesheets/".to_string()
    }
}
