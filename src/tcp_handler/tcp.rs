use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use super::handler::RequestHandler;

pub struct TcpHandler {
    address: String,
}

impl TcpHandler {
    pub fn build(address: &str) -> Self {
        TcpHandler {
            address: address.to_string(),
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to connect to port.");

        println!("Server starting on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || Self::handle_client(stream));
                }

                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buf: [u8; 4096] = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if bytes > 0 {
                    let request = String::from_utf8_lossy(&buf[..bytes]);
                    println!("Got some info!");

                    let response = RequestHandler::handle_request(request.to_string());
                    stream
                        .write_all(response.as_bytes())
                        .expect("Failed to send message.");
                }
            }
            Err(e) => eprintln!("Error recieving request: {}", e),
        }
    }
}
