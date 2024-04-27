mod parser;
mod request;
mod dispatcher;
mod response;
mod server;
mod options;

use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Read;
use anyhow::Ok;
use dispatcher::dispatch;
use options::read_options;
use parser::parse_html;
use response::Response;

use crate::server::Server;

#[derive(Debug)]
struct Context {
    pub base_url: Option<String>
}
impl Context {
    pub fn new() -> Self {
        Self {
            base_url: None
        }
    }
}

fn main() {
    let context : Arc<Mutex<Context>> = Arc::new(Mutex::new(Context::new()));
    read_options(&context);
    let mut app = Server::new();

    app.config("127.0.0.1:4221".to_string(), None);
    app.start(|mut stream| {
        let ctx = context.clone();
        thread::spawn(|| {
            let mut buff = [0; 255];
            let size_buff = stream.read(&mut buff).unwrap();
            match  parse_html(&buff[..size_buff].to_vec()) {
                Some(mut request) => {
                    request.set_context(ctx);
                    dispatch(request, Response::new(stream))
                }
                _ => { Ok(()) }
            }
        });
    });
}
