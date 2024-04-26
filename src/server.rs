use std::net::{TcpListener, TcpStream};

pub struct Server {
  pub addr: Option<String>,
  pub tcp: Option<TcpListener>,
}

impl Server {
  pub fn new() -> Self {
    Self {
      addr: None,
      tcp: None
    }
  }

  pub fn config(&mut self, addr: String) {
    self.tcp = Some(TcpListener::bind(&addr).unwrap());
    self.addr = Some(addr);
    println!("[rapi]: ⚡️ Server started on port 4221 ⚡️");
  }

  pub fn start<F>(self, cb: F)
    where F:
    Fn(TcpStream),
    {
      if self.tcp.is_none() || self.addr.is_none() {
        panic!("Did you forget to run config() first?");
      }

    for stream_incoming in self.tcp.unwrap().incoming() {
      match stream_incoming {
        Ok(stream) => {
          _ = cb(stream);
        }
        _  => {}
      }
    }
  }
}