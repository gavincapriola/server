struct Server { address: String }

impl Server {
  fn new(address: String) -> Self {
    Server { address }
  }
  
  fn start(&self) { 
    println!("Starting server at {}", self.address); 
  }
}

fn main() {
  let server = Server::new("127.0.0.1:8080".to_string());
  server.start();
}
