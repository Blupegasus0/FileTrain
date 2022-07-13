// Client

use std::io::Write;
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
//use std::str::from_utf8;

// Handle all unwrap and error cases after functionality is improved
fn main() {
    let ip_addr = "localhost:3453";
    let msg = String::from("never gonna give you up!");
    tcp_send_msg(&ip_addr, &msg);

    // query file path from user
    let file_path = Path::new("/home/obsidian/Projects/rust/FileTrain/client/test.txt");
    //tcp_send_file(&ip_addr, &file_path);

}

// Send file over a tcp stream
fn tcp_send_file(ip_addr: &str, file_path: &Path) {
    // Create stream 
    let mut stream = TcpStream::connect(ip_addr).unwrap();
    // Get file and convert it into bytes
    let file = get_file(file_path);
    //println!("{}",get_file(file_path));

    // Send message and notify client
    stream.write(file.as_bytes()).unwrap();
    println!("File sent");
}

// Open a file and handle possible errors
fn get_file(file_path: &Path) -> String {
    // Open file
    std::fs::read_to_string(file_path).unwrap()
}

// Send a message over a tcp stream
fn tcp_send_msg(ip_addr: &str, msg: &String) {
    // Create stream and msg
    let mut stream = TcpStream::connect(ip_addr).unwrap();
    let msg = msg.as_bytes();

    // Send message and notify client
    stream.write(msg).unwrap();
    println!("Sent message");
}
