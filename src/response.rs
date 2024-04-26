use std::{fmt::format, io::Write, net::TcpStream};
use anyhow::{Ok, Result};

pub struct ResponseData {
  status: u16,
  body: String
}

pub struct Response {
  pub stream: TcpStream,
  data: ResponseData
}

impl Response {
  pub fn new(stream: TcpStream) -> Self {
    Self {
      stream,
      data: ResponseData {
        status: 200,
        body: "OK".to_string()
      }
    }
  }

  pub fn status(mut self, status_code: u16) -> Self {
    self.data.status = status_code;
    self
  }

  pub fn send(&mut self, body: String) -> Result<()> {
    self.data.body = body;
    let encoded_body = self._encode_();
    self._send_(encoded_body)
  }

  fn _send_(&mut self, data: Vec<u8>) -> Result<()> {
    self.stream.write_all(&data)?;
    self.stream.flush()?;

    Ok(())
  }

  fn _encode_(&mut self) -> Vec<u8> {
    let raw_response = format!("HTTP/1.1 {} {}\r\n\r\n", self.data.status, self.data.body);
    raw_response.as_bytes().to_vec()
  }
}