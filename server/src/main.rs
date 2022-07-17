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

// List of data transfer types
enum data_type {
    file,
    text,
    invalid,
}

fn handle_connection(mut stream: TcpStream) {
    // Read in buffer and handle any errors
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Recieve data so this method knows what type of data its is recieving
    // and use that data to choose which function to call.

    // Define possible data prefixes
    let file = String::from("FILE");
    let text = String::from("TEXT");
    let prefix = String::from_utf8((&buffer[..4]).to_vec()).unwrap();

    // Assign the data prefix to the appropriate enum type
    /*let data_type = match &prefix {
        file => data_type::file,
        text => data_type::text,
        _ => data_type::invalid,
    };*/


    let data_type;
    if &prefix == &file {
        data_type = data_type::file;
    } else if &prefix == &text {
        data_type = data_type::text;
    } else {
        data_type = data_type::invalid;
    }
    println!("{}", prefix);


    // After the metadata is read, remove it appropriately and pass it to the 
    // appropriate function.

    // Remove the prefix from the data`

    // Pass the data to the appropriate function
    match data_type {
        data_type::file => recieve_file(&buffer),
        data_type::text => recieve_text(&buffer),
        data_type::invalid => panic!("Invalid sufix"),
    }
}

fn recieve_file(&buffer: &[u8; 1024]) {
    // If the data is the first packet then:
        // Create new file and give it the name recieved
        let file_name = "recieved.txt";

        let mut file = std::fs::File::options()
        .append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // Append the buffer to the file
    file.write(&buffer);
    //file.write(b"appended");
    println!("new buffer");
    
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