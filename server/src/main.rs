// Server

use std::io::{Read, Write};
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

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // Recieve data so this method knows what type of data its is recieving
    // and use that data to choose which function to call.

    // After the metadata is read, remove it appropriately and pass it to the 
    // appropriate function.

    // if the data is text then:
    recieve_text(&buffer);

    // if the data is a file then:
    recieve_file(&buffer);
}

fn recieve_file(&buffer: &[u8; 1024]) {
    // If the data is the first packet then:
        // Create new file and give it the name recieved
        let file_name = "recieved.txt";

        //let file = std::fs::new(file_name)
        let mut file = std::fs::File::options()
        .append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // Append the buffer to the file
    file.write_all(&buffer);
    //file.write(b"appended");
    
}


fn recieve_text(buffer: &[u8; 1024]) {
    // recieve and print text
    println!("Message: {}", String::from_utf8_lossy(&buffer[..]));
}

// Exicute a command sent by the client
fn recieve_cmd(buffer: &[u8; 1024]) {}

// Take in mouse and keyboard input from the client
// Using UDP... ??
fn recieve_input(){}