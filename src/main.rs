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
