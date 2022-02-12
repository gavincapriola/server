use super::http::{ Request, Response, StatusCode, Method };
use super::server::Handler;

pub struct WebSiteHandler { public_path: String }

impl WebSiteHandler {
  pub fn new(public_path: String) -> Self {
    WebSiteHandler { public_path }
  }
}

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, Some("<h1>Hello, world!</h1>".to_string())),
        _ => Response::new(StatusCode::NotFound, None),
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}
