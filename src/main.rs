struct Server { address: String }
struct Request { 
  method: String,
  path: String,
  query_string: String,
}

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
