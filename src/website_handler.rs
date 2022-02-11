use super::http::{ Request, Response, StatusCode };
use super::server::Handler;

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, _request: &Request) -> Response {
    Response::new(StatusCode::Ok, Some("<h1>Hello, world!</h1>".to_string()))
  }
}
