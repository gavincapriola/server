use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener; 

pub struct Server { address: String }

impl Server {
  pub fn new(address: String) -> Self {
    Server { address }
  }

  pub fn start(self) { 
    println!("Starting server at {}", self.address); 

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              
              match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!()
                },
                Err(e) => println!("Failed to parse a request: {}", e),
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
