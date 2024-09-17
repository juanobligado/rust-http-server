
fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.listen();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
        }
    }

    fn listen(&self) {
        println!("Listening on {}", self.address);
    }
}