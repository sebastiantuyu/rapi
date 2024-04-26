use std::collections::HashMap;

pub struct RawRequest {
  pub parsed: Vec<Vec<String>>,
  pub headers: HashMap<String, String>
}

impl RawRequest {
    pub fn new(parsed: Vec<Vec<String>>, headers: HashMap<String, String>) -> Self {
      Self { parsed, headers }
    }

    pub fn to_request(&mut self) -> Option<Request> {
      if self.parsed.len() < 1 {
        return None;
      }
      dbg!(&self.parsed);
      Some(Request {
        method: self.parsed[0][0].to_string(),
        path: self.parsed[0][1].to_string(),
        // host: self.parsed[1][1].to_string(),
        params: Vec::new(),
        headers: self.headers.clone()
      })
    }
}

#[derive(Debug)]
pub struct  Request {
  pub method: String,
  pub path: String,
  // pub host: String,
  pub params: Vec<String>,
  pub headers: HashMap<String, String>
}


impl Request {
  pub fn add_param(&mut self, param: String) {
    self.params.push(param);
  }
}