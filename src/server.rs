use crate::http::{ Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on {}", self.address);
        // infinite loop
        'main_loop: loop {
            match listener.accept() {
                Ok((mut _socket, _addr)) => {
                    let mut buffer = [0; 1024];
                    match _socket.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // if the request is valid, we print the request
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>It works!</h1>".to_string())
                                    )
                                }
                                Err(e) => {
                                    // if the request is invalid, we print the error
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
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
