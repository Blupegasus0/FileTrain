// Client

use anyhow::anyhow;
use chacha20poly1305::{    
    aead::{stream, Aead, NewAead},                                                                                                                                                            
    XChaCha20Poly1305,    
};    
use rand::{Rng, RngCore, rngs::OsRng}; 
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs;
use std::path::Path;

const BUFFER_SIZE: usize = 1024;

// Handle all unwrap and error cases after functionality is improved

fn main() -> Result<(), anyhow::Error> {
    let ip_addr = "localhost:3453";
    let msg = String::from("never gonna give you up!");
    //tcp_send_msg(&ip_addr, &msg);

    // query file path from user
    let file_path = Path::new("/home/obsidian/Projects/rust/FileTrain/client/test.txt");
    tcp_send_file(&ip_addr, &file_path)?;

    Ok(())

}

// Send file over a tcp stream
fn tcp_send_file(ip_addr: &str, file_path: &Path)
-> Result<(), anyhow::Error> {
    // Create stream 
    let mut stream = TcpStream::connect(ip_addr).unwrap();

    // Append data type(FILE) and file name 
    // to the beginning of the file

    //////////////////////
    // Encrypt the file //
    //////////////////////

    // Create and fill test key and nonce
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    println!("key: {}", key);
    println!("nonce: {}", nonce);
    
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = fs::File::open(file_path)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|e| anyhow!("Encryping large file: {}", e))?;
            
            stream.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|e| anyhow!("Encryping large file: {}", e))?;
            
            stream.write(&ciphertext)?;
            break;
        }
    }

    // Send message and notify client
    
    // Disabled. Stream being sent from encrypt function
    //stream.write(&file).unwrap();
    println!("File sent");

    Ok(())
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
    let is_paired = false;
    // Send a signal to the server that {device name} is 
    // trying to connect.
    // If the server accepts then set "is_paired" to true

    // This is where the key and nonce are shared
    
    
    // return the value of is_paired
    //is_paired
    is_paired

}

// Open a file and handle possible errors
fn get_file(file_path: &Path) -> Vec<u8> {
    // Open file
    std::fs::read(file_path).unwrap()
}
