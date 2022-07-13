// Server

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::Path;

fn main() {
    // create listener and bind it to localhost port 7878
    let listener = TcpListener::bind("localhost:3453").unwrap();

    // listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> [u8; 1024] {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    buffer
}

fn print_msg (buffer: &[u8]) {
    println!("Message: {}", String::from_utf8_lossy(&buffer[..]));
}
