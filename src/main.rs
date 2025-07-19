#![allow(unused_imports)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::result::Result::Ok;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let accept_result = listener.accept().await;
        match accept_result {
            Ok((mut stream, addr)) => {
                // Handle the stream here
                println!("Accepted connection from: {}", addr);

                tokio::spawn(async move {
                    let mut buf = [0; 512];
                    loop {
                        let read_count = stream.read(&mut buf).await.unwrap();
                        if read_count == 0 {
                            break;
                        }
        
                        stream.write(b"+PONG\r\n").await.unwrap();
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
