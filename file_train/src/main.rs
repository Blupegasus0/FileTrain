use anyhow::anyhow;
use chacha20poly1305::{    
    aead::{stream, NewAead},                                                                                                                                                            
    XChaCha20Poly1305,    
};    
use std::io::{Read, Write};
use std::net::TcpListener;
use std::fs::File;
use std::sync::mpsc;
use std::thread;

pub mod server;
pub mod client;

use crate::client::client::run_client;
use crate::server::server::run_server;



const BUFFER_SIZE: usize = 1024;
const PORT: u16 = 3453;

fn main() {
    let server = thread::spawn(|| {
        run_server();
    });

    let client = thread::spawn(|| {
        run_client();
    });


    server.join().unwrap(); // HANDLE ERRORS
    client.join().unwrap(); // HANDLE ERRORS
}



