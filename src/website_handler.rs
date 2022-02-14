use super::http::{ Request, Response, StatusCode, Method };
use super::server::Handler;
use std::fs;

pub struct WebSiteHandler { public_path: String }

impl WebSiteHandler {
  pub fn new(public_path: String) -> Self {
    WebSiteHandler { public_path }
  }

  fn read_file(&self, file_path: &str) -> Option<String> {
    let path = format!("{}/{}", self.public_path, file_path);
    fs::read_to_string(path).ok()
  }
}

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
        _ => Response::new(StatusCode::NotFound, None),
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}
