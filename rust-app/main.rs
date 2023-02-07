use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
