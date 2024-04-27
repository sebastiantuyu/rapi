use std::{env::args_os, sync::{Arc, Mutex}};

use crate::Context;


pub fn read_options(ctx: &Arc<Mutex<Context>>) -> Option<String> {
  let args: Vec<String> = args_os()
      .map(|arg| arg.into_string().unwrap_or_else(|os_string| {
          os_string.to_string_lossy().to_string()
      }))
      .collect();
  let sz = args.len();

  for (_, argument) in args.iter().enumerate() {
      let arg = argument;
      if arg.starts_with("--") {
          let option: Vec<String> = arg.split("--").map(|v| v.to_string()).collect();
          match option[1].as_str() {
              "directory" => {
                  if sz <= 2 {
                      panic!("Missing arguments for [directory]");
                  }
                  ctx.lock().unwrap().base_url = Some(args[2].to_string());
                  return Some(option[1].to_string())
              }
              _ => { return None; }
          }
      }
  }
  None
}
