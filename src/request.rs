use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::Context;

pub struct RawRequest {
  pub parsed: Vec<Vec<String>>,
  pub headers: HashMap<String, String>,
  pub body: String
}

impl RawRequest {
    pub fn new(parsed: Vec<Vec<String>>, headers: HashMap<String, String>, body: String) -> Self {
      Self { parsed, headers, body }
    }

    pub fn to_request(&mut self) -> Option<Request> {
      if self.parsed.len() < 1 {
        return None;
      }
      Some(Request {
        body: self.body.clone(),
        method: self.parsed[0][0].to_string(),
        path: self.parsed[0][1].to_string(),
        params: Vec::new(),
        headers: self.headers.clone(),
        context: None
      })
    }
}

#[derive(Debug)]
pub struct  Request {
  pub body: String,
  pub method: String,
  pub path: String,
  pub params: Vec<String>,
  pub headers: HashMap<String, String>,
  pub context: Option<Arc<Mutex<Context>>>
}


impl Request {
  pub fn add_param(&mut self, param: String) {
    self.params.push(param);
  }
  pub fn set_context(&mut self, ctx: Arc<Mutex<Context>>) {
    self.context = Some(ctx);
  }

  pub fn get_context(&mut self) -> Option<String> {
    match &self.context {
      Some(ctx) => { return ctx.lock().unwrap().base_url.clone() },
      _ => { None }
    }
  }
}