use chunked_transfer::Encoder;

mod utils;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

use crate::utils::response::Response;

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

        render_css(&css)
    } else if uri(&buffer).ends_with("png") {
        let path = match uri(&buffer).split("/").last().unwrap() {
            "two.png" => "src/client/images/two.png",
            "betteryellowzero.png" => "src/client/images/betteryellowzero.png",
            "betterredzero.png" => "src/client/images/betterredzero.png",
            "five.png" => "src/client/images/five.png",
            "logo.png" => "src/client/images/logo.png",
            _ => "src/client/images/two.png",
        };

        render_image(path)
    } else {
        Response::new(&buffer).await.render().as_bytes().to_vec()
    };

    stream.write(&response).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}

pub fn render_css(contents: &str) -> Vec<u8> {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK".to_string(),
        contents.len(),
        contents
    )
    .as_bytes()
    .to_vec()
}

fn render_image(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_end(&mut buf).unwrap();

    let mut encoded = Vec::new();
    {
        let mut encoder = Encoder::with_chunks_size(&mut encoded, 32);
        encoder.write_all(&buf).unwrap();
    }

    let headers = [
        "HTTP/1.1 200 OK",
        "Content-type: image/jpeg",
        "Transfer-Encoding: chunked",
        "\r\n",
    ];

    let mut response = headers.join("\r\n").to_string().into_bytes();
    response.extend(encoded);

    response
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
