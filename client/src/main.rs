// Client

use std::io::Write;
use std::net::TcpStream;
use std::fs;
use std::path::Path;
//use std::str::from_utf8;

// Handle all unwrap and error cases after functionality is improved
fn main() {
    let ip_addr = "localhost:3453";
    let msg = String::from("never gonna give you up!");
    //tcp_send_msg(&ip_addr, &msg);

    // query file path from user
    let file_path = Path::new("/home/obsidian/Projects/rust/FileTrain/client/test.txt");
    tcp_send_file(&ip_addr, &file_path);

}

// Send file over a tcp stream
fn tcp_send_file(ip_addr: &str, file_path: &Path) {
    // Create stream 
    let mut stream = TcpStream::connect(ip_addr).unwrap();
    // Get file and convert it into bytes
    let file = get_file(file_path);

    // Append data type(FILE), file size(bytes) and
    // file name to the end of the file

    // Encrypt the file ?

    // Send message and notify client
    stream.write(&file).unwrap();
    println!("File sent");
}

// Open a file and handle possible errors
fn get_file(file_path: &Path) -> Vec<u8> {
    // Open file
    std::fs::read(file_path).unwrap()
}

// Send a message over a tcp stream
fn tcp_send_msg(ip_addr: &str, msg: &String) {
    // Create stream and msg
    let mut stream = TcpStream::connect(ip_addr).unwrap();
    let msg = msg.as_bytes();

    // Append data type (TEXT) and data size (bytes) to 
    // the beginning of the message 
    
    // Encrypt the message ?

    // Send message and notify client
    stream.write(msg).unwrap();
    println!("Sent message");
}

fn pair(ip_addr: &str,) -> bool {
    let is_paired: bool;
    // Send a signal to the server that {device name} is 
    // trying to connect.
    // If the server accepts then set "is_paired" to true
    
    
    // return the value of is_paired
    //is_paired
    true

}
