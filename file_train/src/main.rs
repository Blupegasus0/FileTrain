use std::thread;
// use std::sync::mpsc;

pub mod server;
pub mod client;

use crate::client::client::run_client;
use crate::server::server::run_server;


const BUFFER_SIZE: usize = 1024;
const METADATA: usize = 20;
const NONCE: usize = 19;
const PORT: u16 = 3453;

// List of data transfer types
enum DataType {
    File,
    Pair,
    Text,
    Invalid,
}


fn main() {

    let server_handle = thread::spawn(|| {
        loop {
            match run_server() {
                Err(e) => println!("Error: {e}"), // Displaye error to user
                Ok(_) => println!("Reception Success"),
            }
            break; //TEMPORARY
        }
    });

    let client_handle = thread::spawn(|| {
        match run_client() {
            Err(e) => println!("Error: {e}"), // Display error to user
            Ok(_) => println!("Transmission Success"),
        }
    });


    client_handle.join().unwrap(); // HANDLE ERRORS


    server_handle.join().unwrap(); // HANDLE ERRORS
}



