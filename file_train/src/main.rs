// use anyhow::anyhow;
// use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

pub mod server;
pub mod client;

use crate::client::client::run_client;
use crate::server::server::run_server;


const BUFFER_SIZE: usize = 1024;
const METADATA: usize = 20;
const NONCE: usize = 19;
const PORT: u16 = 3453;

enum DataType {
    File,
    Pair,
    Text,
    Invalid,
}


fn main() {

    let m = Arc::new(Mutex::new(0));

        let m_server = Arc::clone(&m);
        let server_handle = thread::spawn(move || {
            loop {
                // let _mutex = m_server.lock().unwrap();
                run_server();
                break; //TEMPORARY
            }
        });

    let m_client = Arc::clone(&m);
    let client_handle = thread::spawn(move || {
        // let _mutex = m_client.lock().unwrap();
        run_client();

    });


    client_handle.join().unwrap(); // HANDLE ERRORS


    server_handle.join().unwrap(); // HANDLE ERRORS
}



