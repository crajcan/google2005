use std::fs::File;
use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

extern crate google2005;
use google2005::search_for_web_results;

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

    let search = b"GET /search";

    let (status_line, contents) = if buffer.starts_with(search) {
        let query = &buffer[9..];
        let string_query = String::from_utf8_lossy(query);

        println!("query: {}", string_query);

        let contents = search_for_web_results(&string_query).await;
        ("HTTP/1.1 200 OK\r\n\r\n", contents.unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not Found".to_string())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    println!("Response: {}", response);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}
