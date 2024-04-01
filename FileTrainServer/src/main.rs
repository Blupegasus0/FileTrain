pub mod server;

use crate::server::server::run_server;


const BUFFER_SIZE: usize = 1024;
const KEY_LEN: usize = 32;
const NONCE: usize = 19;
const METADATA: usize = NONCE+1; 
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
    let _server = match run_server() {
        Err(e) => println!("Error: {e}"), // Display error to user
        Ok(_) => println!("Reception Success"),
    };
}



