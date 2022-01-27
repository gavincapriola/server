struct Server { address: String }
struct Request { 
  method: Method,
  path: String,
  query_string: String,
}

enum Method { 
  GET,
  POST,
  PUT,
  DELETE,
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
  let get = Method::GET;
  let delete = Method::DELETE;
  let post = Method::POST;
  let put = Method::PUT;
  
  let server = Server::new("127.0.0.1:8080".to_string());
  server.start();
}
