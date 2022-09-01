// Client

use anyhow::anyhow;
use chacha20poly1305::{    
    aead::{stream, Aead, NewAead},                                                                                                                                                            
    XChaCha20Poly1305,    
};    
use rand::{Rng, RngCore, rngs::OsRng}; 
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;

const BUFFER_SIZE: usize = 1024;
const PORT: u16 = 3453;

// Handle all unwrap and error cases after functionality is improved

fn main() -> Result<(), anyhow::Error> {
    let key = [0u8; 32];
    let nonce = [0u8; 19];
    let file_path = "test.txt";
    let ip_addr = String::from("localhost");

    encrypt_tcp(file_path, &key, &nonce, &ip_addr)?;

    Ok(())

}

fn encrypt_tcp(
    source_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
    ip_addr: &String,
) -> Result<(), anyhow::Error> {
    // create socket address
    let socket = format!("{}:{}", ip_addr, PORT);

    // Initialize encryption variables
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    let mut buffer = [0u8; BUFFER_SIZE];

    let mut source_file = File::open(source_file_path)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        println!("{}", read_count);

        if read_count == BUFFER_SIZE {
            // If the buffer is full then expect more data
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|e| anyhow!("Encryping large file: {}", e))?;
            
            // Connect to the stream
            let mut stream = TcpStream::connect(&socket).unwrap();
            //  Write message to the stream
            stream.write(&ciphertext).unwrap();

        } else {
            // If the buffer is not full then send the ending packet
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|e| anyhow!("Encryping large file: {}", e))?;
            
            // Connect to the stream
            let mut stream = TcpStream::connect(&socket).unwrap();
            //  Write message to the stream
            stream.write(&ciphertext).unwrap();

            break;
        }
    }

    Ok(())
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
