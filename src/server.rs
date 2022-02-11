use crate::http::{ Request, Response, StatusCode, ParseError };
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener; 

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server { address: String }

impl Server {
  pub fn new(address: String) -> Self {
    Server { address }
  }

  pub fn start(self, mut handler: impl Handler) { 
    println!("Starting server at {}", self.address); 

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
              };

              if let Err(e) = response.send(&mut stream) {
                println!("Failed to send a response: {}", e);
              }
            }
            Err(e) => println!("Failed to read from connection: {}", e),
          }
        }
        Err(e) => println!("Failed to establish a connection: {}", e),
      }
    }
  }
}
