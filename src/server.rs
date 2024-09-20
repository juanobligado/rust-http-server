use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl dyn Handler {

}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub fn listen(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on {}", self.address);
        // infinite loop
        loop {
            match listener.accept() {
                Ok((mut _socket, _addr)) => {
                    let mut buffer = [0; 1024];
                    match _socket.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut _socket){
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from connection: {}", e);
                        }
                    }
                    println!("Connection established");
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}
