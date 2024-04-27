use std::net::{TcpListener, TcpStream};

pub struct ServerOptions {
  pub base_url: String,
}

pub struct Server {
  pub addr: Option<String>,
  pub tcp: Option<TcpListener>,
  pub options: Option<ServerOptions>
}

impl Server {
  pub fn new() -> Self {
    Self {
      addr: None,
      tcp: None,
      options: None
    }
  }

  pub fn config(&mut self, addr: String, base_url: Option<String>) {
    self.tcp = Some(TcpListener::bind(&addr).unwrap());
    self.addr = Some(addr);
    if self.options.is_none() && !base_url.is_none() {
      self.options = Some(ServerOptions {
        base_url: base_url.unwrap()
      })
    }
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