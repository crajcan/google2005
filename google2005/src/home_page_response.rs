use askama::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct HomePageResponse;
