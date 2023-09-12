use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
extern crate google2005;
use google2005::home_page_response::HomePageResponse;

use askama::Template;
mod utils;

use crate::utils::response::Response as Google2005Response;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_method() {
        &Method::GET | &Method::HEAD => (),
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    if req.get_path() == "/" {
        let homepage = HomePageResponse::new().render().unwrap();

        Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(homepage))
    } else if req.get_path() == "/search" {
        Ok(Google2005Response::new(req.get_query_str()).render())
    } else {
        Ok(
            Response::from_status(StatusCode::NOT_FOUND).with_body_text_plain(
                &format!(
                    r#"The requested page: "{}", with path: {}, could not be found\n"#,
                    req.get_url_str(),
                    req.get_path()
                ),
            ),
        )
    }
}
