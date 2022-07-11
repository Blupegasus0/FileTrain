use std::io::Write;
use std::net::TcpStream;
//use std::str::from_utf8;

fn main() {
    let ip_addr = "localhost:3453";
    let msg = String::from("never gonna give you up!");
    tcp_send_msg(&ip_addr, &msg);
}

fn tcp_send_msg(ip_addr: &str, msg: &String) {
    // Create stream and msg
    let mut stream = TcpStream::connect(ip_addr).unwrap();
    let msg = msg.as_bytes();

    // Send message and notify client
    stream.write(msg).unwrap();
    println!("Sent message");
}
