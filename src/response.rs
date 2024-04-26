use std::{io::Write, net::TcpStream};
use anyhow::{Ok, Result};

pub struct ResponseData {
  status: u16,
  body: Option<String>
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
        body: None
      }
    }
  }

  pub fn status(mut self, status_code: u16) -> Self {
    self.data.status = status_code;
    self
  }

  pub fn send(&mut self, body: Option<String>) -> Result<()> {
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
    let response: String;
    let header_response = {
      match self.data.status {
        200 => { "OK" }
        _ => { "Not Found" }
      }
    };
    let mut raw_response = [format!("HTTP/1.1 {} {}", self.data.status, header_response)].to_vec();

    if let Some(body) = &self.data.body {
      raw_response.extend_from_slice(&[
        format!("Content-Type: text/plain"),
        format!("Content-Length: {}", body.len()),
        format!(""),
        format!("{}", body),
      ]);
    }

    if raw_response.len() == 1 {
      response = raw_response[0].to_string() + "\r\n\r\n";
    } else {
      response = raw_response.join("\r\n");
    }
    response.as_bytes().to_vec()
  }
}