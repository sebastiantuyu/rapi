use std::collections::HashMap;

use crate::request::{RawRequest, Request};

pub fn parse_html(request: &Vec<u8>) -> Option<Request> {
  let raw_request = String::from_utf8_lossy(&request);
  let mut char_request = raw_request
    .chars()
    .peekable();

  let mut body: String = "".to_string();
  let mut sections: Vec<String>= Vec::new();
  let mut sections_processed: Vec<Vec<String>> = Vec::new();
  let mut headers: HashMap<String, String> = HashMap::new();

  let mut cursor: usize = 0;
  while let Some(char) = char_request.next() {
    match char {
        '\r' => {
          // consume '\n'
          char_request.next();
          sections_processed.push(Vec::new());

          sections_processed[cursor] = sections[cursor]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

          if sections_processed[cursor][0].contains(&":".to_string()) {
            // this is a header
            headers.insert(
              sections_processed[cursor][0].replace(":", ""),
              sections_processed[cursor][1].clone()
            );
          }

          match char_request.peek() {
            // actual consume of the body
            Some(body_char) => {
              if body_char == &'\r' {
                // consume '\r'
                char_request.next();
                // consume '\n'
                char_request.next();
                while let Some(bc) = char_request.peek() {
                  if bc == &'\r' {
                    break;
                  }
                  body += &char_request.next().unwrap().to_string();
                }
              }
            }
            _ => {}
          }
          cursor += 1;
          sections.push("".to_string());
        }
        _ => {
          if sections.len() == 0 {
            sections.push("".to_string());
          }
          let current = &sections[cursor];
          sections[cursor] = current.to_owned() +  &char.to_string();
        }
    }
  }
  RawRequest::new(sections_processed, headers, body).to_request()
}