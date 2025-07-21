#![allow(unused_imports)]
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::result::Result::Ok;
use tokio::net::TcpListener;

use crate::resp::Value;

mod resp;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let accept_result = listener.accept().await;
        match accept_result {
            Ok((stream, addr)) => {
                println!("Accepted connection from: {}", addr);

                tokio::spawn(async move {
                    let mut handler = resp::RespHandler::new(stream);
                    loop {
                        let value = handler.read_value().await.unwrap();
                        println!("read_value: {:?}", value);
                        let response = if let Some(v) = value {
                            let (command, args) = extract_command(v).unwrap();
                            match command.as_str() {
                                "ping" => Value::SimpleString("PONG".to_string()),
                                "echo" => args.first().unwrap().clone(),
                                c => panic!("Cannot handle command {}", c),
                            }
                        } else {
                            break;
                        };
                        println!("Sending value {:?}", response);
                        handler.write_value(response).await.unwrap();
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                break;
            }
        }
    }
}

fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                unpack_bulk_str(a.first().unwrap().clone())?,
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

fn unpack_bulk_str(value: Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Expected command to be a bulk string"))
    }
}
