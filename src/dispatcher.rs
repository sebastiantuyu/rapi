use crate::{request::Request, response::Response};
use anyhow::{Ok, Result};

pub fn dispatch(request: Request, response: Response) -> Result<()> {
  match request.path.as_str() {
    "/" => {
      response.status(200).send("OK".to_string())?
    }
    _ => {
      response.status(404).send("Not Found".to_string())?
    }
  }
  // if request.path == "/" {
  //   response.status(200).send("<html></html>".to_string())?
  // }

  Ok(())
}