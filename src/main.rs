use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

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

    let query = req.get_path();

    match query {
        "/search?q=" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(Google2005Response::new(&query.as_bytes()).render())),
        // .as_bytes().to_vec()
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}

use askama::Template;

const SEARCH_URI: &'static str = "GET /search?q=";

pub struct Google2005Response {
    contents: String,
    status_line: String,
}

impl Google2005Response {
    pub fn new(buffer: &[u8]) -> Google2005Response {
        if !buffer.starts_with(SEARCH_URI.as_bytes()) {
            return Google2005Response {
                contents: "".to_string(),
                status_line: format!("HTTP/1.1 404 Not Found"),
            };
        }

        match Self::html_search_response(&Google2005Request::query(&buffer)) {
            Ok(contents) => Google2005Response {
                contents,
                status_line: "HTTP/1.1 200 OK".to_string(),
            },
            Err(e) => Google2005Response {
                contents: format!("{}", e),
                status_line: format!("HTTP/1.1 {} {}", e.status_code, e.status),
            },
        }
    }

    fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
        let search_results = google2005::google(query)?;

        Ok(search_results.render()?)
    }

    pub fn render(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            self.contents.len(),
            self.contents
        )
    }
}

struct Google2005Request;

impl Google2005Request {
    pub fn query(buffer: &[u8]) -> String {
        let after_equals = &buffer[14..];
        let until_space = after_equals.split(|c| *c == b' ').next().unwrap();
        let string_query = String::from_utf8_lossy(until_space);

        string_query.to_string()
    }
}
