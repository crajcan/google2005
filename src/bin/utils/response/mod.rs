// use crate::utils::request;
// use askama::Template;

// const SEARCH_URI: &'static str = "GET /search?q=";

// pub struct Response {
//     contents: String,
//     status_line: String,
// }

// impl Response {
//     pub async fn new(buffer: &[u8]) -> Response {
//         if !buffer.starts_with(SEARCH_URI.as_bytes()) {
//             return Response {
//                 contents: "".to_string(),
//                 status_line: format!("HTTP/1.1 404 Not Found"),
//             };
//         }

//         match Self::html_search_response(&request::query(&buffer)).await {
//             Ok(contents) => Response {
//                 contents,
//                 status_line: "HTTP/1.1 200 OK".to_string(),
//             },
//             Err(e) => Response {
//                 contents: format!("{}", e),
//                 status_line: format!("HTTP/1.1 {} {}", e.status_code, e.status),
//             },
//         }
//     }

//     async fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
//         let search_results = google2005::google(query).await?;

//         Ok(search_results.render()?)
//     }

//     pub fn render(&self) -> String {
//         format!(
//             "{}\r\nContent-Length: {}\r\n\r\n{}",
//             self.status_line,
//             self.contents.len(),
//             self.contents
//         )
//     }
// }
