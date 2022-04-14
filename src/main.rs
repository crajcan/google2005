use askama::Template;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

extern crate google2005;

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

    let search = b"GET /search?q=";

    let (status_line, contents) = if buffer.starts_with(search) {
        let contents = html_search_response(&query(&buffer)).await;
        ("HTTP/1.1 200 OK", contents.unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Not Found".to_string())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}

async fn html_search_response(query: &str) -> Result<String, google2005::Google2005Error> {
    let search_results = google2005::google(query).await?;

    Ok(search_results.render()?)
}
