// use anyhow::anyhow;
// use std::sync::mpsc;
use std::thread;

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
            run_server();
            break; //TEMPORARY
        }
    });

    let client_handle = thread::spawn(|| {
        run_client();
    });


    client_handle.join().unwrap(); // HANDLE ERRORS


    server_handle.join().unwrap(); // HANDLE ERRORS
}



