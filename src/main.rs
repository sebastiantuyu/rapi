// Uncomment this block to pass the first stage
use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("[rapi]: ⚡️ Server started on port 4221 ⚡️");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                _ = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
                _ = stream.flush();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
