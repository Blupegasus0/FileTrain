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
const NONCE: usize = 19;
const PORT: u16 = 3453;

fn main() {
    use std::io;
    let mut input = String::new();


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



