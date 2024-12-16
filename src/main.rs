#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener
};

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = [0; 512];

                loop {
                    let read_count = _stream.read(&mut buf).unwrap();
                    println!("{}", read_count);
                    if read_count == 0 {
                        break;
                    }
                    _stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    println!("ending mainnnnn");
}
