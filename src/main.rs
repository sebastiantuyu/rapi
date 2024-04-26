mod parser;
mod request;
mod dispatcher;
mod response;

use std::{io::{Read, Write}, net::TcpListener};

use crate::{dispatcher::dispatch, parser::parse_html, response::Response};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("[rapi]: ⚡️ Server started on port 4221 ⚡️");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buff = [0; 255];
                let size_buff = stream.read(&mut buff).unwrap();
                let request = parse_html(&buff[..size_buff].to_vec());
                match request {
                    Some(req) => {
                        _ = dispatch(req, Response::new(stream));
                    }
                    _ => {
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
