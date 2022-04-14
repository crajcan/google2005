use askama::Template;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

extern crate google2005;

const SEARCH_URI: &'static str = "GET /search?q=";

#[tokio::main]
async fn main() {
    let listener: tokio::net::TcpListener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

fn query(buffer: &[u8]) -> String {
    let after_equals = &buffer[14..];
    let until_space = after_equals.split(|c| *c == b' ').next().unwrap();
    let string_query = String::from_utf8_lossy(until_space);
    println!("***** string query: {}******", string_query);

    string_query.to_string()
}

async fn handle_connection(mut stream: TcpStream) {
    println!("handling connection");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    let response = Response::new(&buffer).await;

    stream.write(response.render().as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}

async fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
    let search_results = google2005::google(query).await?;

    Ok(search_results.render()?)
}

struct Response {
    contents: String,
    status_line: String,
}

impl Response {
    async fn new(buffer: &[u8]) -> Response {
        if !buffer.starts_with(SEARCH_URI.as_bytes()) {
            return Response {
                contents: "".to_string(),
                status_line: format!("HTTP/1.1 404 Not Found"),
            };
        }

        match html_search_response(&query(&buffer)).await {
            Ok(contents) => Response {
                contents,
                status_line: "HTTP/1.1 200 OK".to_string(),
            },
            Err(e) => Response {
                contents: format!("{}", e),
                status_line: format!("HTTP/1.1 {} {}", e.status_code, e.status),
            },
        }
    }

    fn render(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            self.contents.len(),
            self.contents
        )
    }
}
