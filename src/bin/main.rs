mod utils;
use utils::response::Response;

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

async fn handle_connection(mut stream: TcpStream) {
    println!("handling connection");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    let response = Response::new(&buffer).await;

    stream.write(response.render().as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("\n---------------------------------------------------------------\n\n");
}
