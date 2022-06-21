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

    if req.get_path() == "/search" {
        match Google2005Response::new(req.get_query_str())
            .contents()
            .as_str()
        {
            "" => Ok(Response::from_status(StatusCode::UNPROCESSABLE_ENTITY)
                .with_body_text_plain("Please enter a query.")),
            resp => Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(resp)),
        }
    } else {
        Ok(
            Response::from_status(StatusCode::NOT_FOUND).with_body_text_plain(&format!(
                r#"The requested page: "{}", could not be found\n"#,
                req.get_url_str()
            )),
        )
    }
}

use askama::Template;

const SEARCH_URI: &'static str = "q=";
pub struct Google2005Response {
    contents: String,
}

impl Google2005Response {
    pub fn new(query: Option<&str>) -> Google2005Response {
        if query == None {
            return Google2005Response {
                contents: String::from(""),
            };
        }

        let query = query.unwrap();

        if !query.starts_with(SEARCH_URI) {
            return Google2005Response {
                contents: "".to_string(),
            };
        }

        let query = query.trim_start_matches(SEARCH_URI);

        match Self::html_search_response(query) {
            Ok(contents) => Google2005Response { contents },
            Err(e) => Google2005Response {
                contents: format!("{}", e),
            },
        }
    }

    fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
        let search_results = google2005::google(query)?;

        Ok(search_results.render()?)
    }

    fn contents(self) -> String {
        self.contents
    }
}

// struct Google2005Request;

// impl Google2005Request {
//     pub fn query(query: &str) -> String {
//         let bytes = query.as_bytes();
//         let after_equals = &bytes[14..];
//         let until_space = after_equals.split(|c| *c == b' ').next().unwrap();
//         let string_query = String::from_utf8_lossy(until_space);

//         string_query.to_string()
//     }
// }
