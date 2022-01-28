use std::net::TcpListener; // https://doc.rust-lang.org/std/net/struct.TcpListener.html

pub struct Server { address: String }

impl Server {
  pub fn new(address: String) -> Self {
    Server { address }
  }

  pub fn start(&self) { 
    println!("Starting server at {}", self.address); 

    let listener = TcpListener::bind(&self.address).unwrap();
  }
}
