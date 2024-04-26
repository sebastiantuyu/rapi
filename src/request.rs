pub struct RawRequest {
  pub parsed: Vec<Vec<String>>
}

impl RawRequest {
    pub fn new(parsed: Vec<Vec<String>>) -> Self {
      Self { parsed }
    }

    pub fn to_request(&mut self) -> Option<Request> {
      if self.parsed.len() < 1 {
        return None;
      }
      Some(Request {
        method: self.parsed[0][0].to_string(),
        path: self.parsed[0][1].to_string(),
        host: self.parsed[1][1].to_string()
      })
    }
}

#[derive(Debug)]
pub struct  Request {
  pub method: String,
  pub path: String,
  pub host: String
}
