mod parser;
mod request;
mod dispatcher;
mod response;
mod server;

use std::thread;
use std::io::Read;
use dispatcher::dispatch;
use parser::parse_html;
use response::Response;

use crate::server::Server;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // println!("[rapi]: ⚡️ Server started on port 4221 ⚡️");

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             let mut buff = [0; 255];
    //             let size_buff = stream.read(&mut buff).unwrap();
    //             let request = parse_html(&buff[..size_buff].to_vec());
    //             match request {
    //                 Some(req) => {
    //                     _ = dispatch(req, Response::new(stream));
    //                 }
    //                 _ => {
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
    let mut app = Server::new();
    app.config("127.0.0.1:4221".to_string());
    app.start(|mut stream| {
        thread::spawn(|| {
            let mut buff = [0; 255];
            let size_buff = stream.read(&mut buff).unwrap();
            let request = parse_html(&buff[..size_buff].to_vec()).unwrap();
            dispatch(request, Response::new(stream))
        });
    });
}
