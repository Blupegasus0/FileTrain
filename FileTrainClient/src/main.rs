use std::thread;
use std::time::Duration;
// use std::sync::mpsc;

pub mod client;

use crate::client::client::run_client;


const BUFFER_SIZE: usize = 1024;
const KEY_LEN: usize = 32;
const METADATA: usize = 20; 
const NONCE: usize = 19;
const TEMP_KEY: [u8; KEY_LEN] = [0u8; KEY_LEN];
const PORT: u16 = 3453;
const ROUNDS: Option<u32> = Some(5);

// List of data transfer types
pub mod data_type {
    pub const PAIR: u8 = 0;
    pub const FILE: u8 = 1;
    pub const TEXT: u8 = 2;
}


fn main() {
        let _client = match run_client() {
            Err(e) => println!("Error: {e}"), // Display error to user
            Ok(_) => println!("Transmission Success"),
        };

}



