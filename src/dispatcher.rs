use crate::{request::Request, response::Response};
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn file_exists(filename: &str) -> bool {
  if let Ok(metadata) = fs::metadata(filename) {
      metadata.is_file()
  } else {
      false
  }
}

type CallbackFn =  Box<dyn Fn(Request, Response) -> Result<()>>;
struct CallbackItem {
  name: Regex,
  callback: CallbackFn,
}

impl CallbackItem {
  pub fn new(name: String, callback: CallbackFn) -> Self {
    Self {
      name: Regex::new(&name).unwrap(),
      callback
    }
  }
}

pub fn dispatch(mut request: Request, response: Response) -> Result<()> {
  let callbacks: Vec<CallbackItem> = vec![
    CallbackItem::new(
      "^/$".to_string(),
      Box::new(|_, resp| {
        resp.status(200).send(None)?;
        Ok(())
      })
    ),
    CallbackItem::new(
      "^/echo/(.+)".to_string(),
      Box::new(|req, resp| {
        resp.status(200).send(Some(req.params[0].to_string()))?;
        Ok(())
      })
    ),
    CallbackItem::new(
      "^/user-agent".to_string(),
      Box::new(|req, resp| {
        let header = req.headers.get("User-Agent");
        match header {
          Some(value) => {
            resp.status(200).send(Some(value.to_string()))?;
          }
          _ => {
            resp.status(400).send(Some(req.params[0].to_string()))?;
          }
        }

        Ok(())
      })
    ),
    CallbackItem::new(
      "^/files/(.+)".to_string(),
      Box::new(|mut req, mut res| {
        let mut base_url = "./static".to_string();
        match req.get_context() {
          Some(v) => { base_url = v.to_string(); }
          _ => {}
        }
        let file_addr = format!("{base_url}/{}", req.params[0].to_string());
        if !file_exists(&file_addr) {
          _ = res.status(404).send(None);
          return Ok(())
        }
        let file = File::open(file_addr).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        _ = res.send_file(buffer);
        Ok(())
      })
    )
  ];

  for callback in callbacks {
    if let Some(captures ) = callback.name.captures(&request.path.clone()) {
      for (i, cap) in captures.iter().enumerate() {
        if i == 0 { continue; }
        match cap {
            Some(parameter) => {
              _ = &request.add_param(parameter.as_str().to_string());
            }
            _ => {}
        }
      }
      return (callback.callback)(request, response);
    }
  }

  // callback 404 if non route matched
  _ = &response.status(404).send(None);
  Ok(())
}