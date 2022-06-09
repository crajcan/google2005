mod utils;
use utils::response::Response;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::fs;
use std::io::BufRead;

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

async fn handle_connection(mut stream: TcpStream) {
    println!("handling connection");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    println!("\nRequest: {}\n", String::from_utf8_lossy(&buffer[..]));

    let response = if uri(&buffer).ends_with("css") {
        println!("fetching css");
        let css = fs::read_to_string("src/client/stylesheets/search.css").unwrap();

        render_css(css)
    } else {
        Response::new(&buffer).await.render()
    };

    println!("\nResponse: {}\n", response);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}

fn first_line(buffer: &[u8]) -> String {
    let mut lines = buffer.lines();
    let first = lines.next().unwrap();

    match first {
        Ok(s) => s,
        Err(_) => "".to_string(),
    }
}

fn uri(buffer: &[u8]) -> String {
    let first_line = first_line(buffer);

    let mut parts = first_line.split_whitespace();

    match parts.next() {
        Some(_s) => parts.next().unwrap_or("").to_string(),
        None => "".to_string(),
    }
}

pub fn render_css(contents: String) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK".to_string(),
        contents.len(),
        contents
    )
}
