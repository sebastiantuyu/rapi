use crate::{request::Request, response::Response};
use anyhow::Result;
use regex::Regex;

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
      "^/echo/(.+)/$".to_string(),
      Box::new(|req, resp| {
        resp.status(200).send(Some(req.params[0].to_string()))?;
        Ok(())
      })
    ),
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
      match (callback.callback)(request, response) {
        Ok(_) => { return Ok(()); }
        Err(_) => { return Ok(()); }
      }
    }
  }

  // callback 404 if non route matched
  _ = &response.status(404).send(None);
  Ok(())
}