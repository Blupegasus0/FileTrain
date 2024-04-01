pub mod server {

    use anyhow::{anyhow, Ok};
    use chacha20poly1305::{    
        aead::{stream, Aead, NewAead, Buffer}, XChaCha20Poly1305,    
    };    
    use core::panic;
    use rand::{RngCore, rngs::OsRng}; 
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::fs::File;
    use pwhash::{sha1_crypt, HashSetup};
    // use std::path::Path;

    use crate::METADATA;
    use crate::NONCE;
    use crate::BUFFER_SIZE;
    use crate::KEY_LEN;
    use crate::PORT;
    use crate::TEMP_KEY;
    use crate::ROUNDS;
    use crate::data_type;
    // use crate::DataType;

    const MISC_HEADERS: usize = 16;


    pub fn run_server() -> anyhow::Result<()> {
        let password = String::from("password");

        decrypt_tcp(&password)?;

        Ok(())
    }


    fn decrypt_tcp(
        // ip_addr: &String
        password: &String
    ) -> anyhow::Result<()> {

        // let the IP always be localhost
        let socket = format!("localhost:{}", PORT);

        // create listener and bind it to the socket
        let listener = TcpListener::bind(socket)
            .expect("Failed to bind to socket");
        let mut buffer = [0; BUFFER_SIZE+METADATA+MISC_HEADERS];

        let output_path = "test_files/output.pdf";
        // let output_path = "test_files/output.txt";

        // Overwrites existing file 
        let mut output_file = File::create(output_path)
            .map_err(|e| anyhow!("Creating output file: {e}"))?;

        // listen for incoming connections
        for stream in listener.incoming() {

            let mut stream = stream?;

            // Read in buffer, locate nonce
            let read_count = stream.read(&mut buffer)?;
            let data_type = buffer[0];
            let nonce_start = METADATA - NONCE; let nonce_end = nonce_start + NONCE;
            let nonce = &buffer[nonce_start..nonce_end];

            // Initialize decryption variables 
            let aead = XChaCha20Poly1305::new(TEMP_KEY[..].into());
            let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.into());

            // Call the pair function when a pair pair_request is received
            let _ = match data_type {
                data_type::PAIR => {
                    pair(&mut stream, &password).expect("pairing failed"); 
                    return Ok(());
                },
                _ => (),
            };


            // if so, break the loop
            // data could also be a command, handle that appropriately

            if read_count == buffer.len() { 
                // If the buffer is full then expect more packets
                let plaintext = stream_decryptor
                    .decrypt_next(&buffer[METADATA..])
                    .map_err(|e| anyhow!("Decrypting large file step 1: {}", e))?;

                // check if the data is a pair_request
                match data_type {
                    data_type::FILE => receive_file(&mut output_file, &plaintext)?,
                    data_type::TEXT => receive_text(&plaintext),
                    _ => panic!("Invalid data_type header received"),
                }

            } else if read_count == 0 {
                // If there is no more data ... end
                println!("no data read");
                break;
            } else {
                // If the buffer is neither empty nor full then this is the last packet
                let plaintext = stream_decryptor
                    .decrypt_last(&buffer[METADATA..read_count])
                    .map_err(|e| anyhow!("Decrypting large file step 2: {:?}", e))?;

                match data_type {
                    data_type::FILE => receive_file(&mut output_file, &plaintext)?,
                    data_type::TEXT => receive_text(&plaintext),
                    _ => panic!("Invalid data_type header received"),
                }
                break;
            }

        }

        Ok(())
    }


    fn receive_file(output_file: &mut File, plaintext: &Vec<u8>) -> anyhow::Result<()> {
        output_file.write(&plaintext)?;
        Ok(())
    }

    fn receive_text(plaintext: &Vec<u8>) {
        // maybe handle the formatting a bit better lol
        println!("{}", String::from_utf8_lossy(plaintext));
    }

    // Execute a command sent by the client
    // fn recieve_cmd(buffer: &[u8; BUFFER_SIZE]) {}

    // Take in mouse and keyboard input from the client
    // Using UDP... ??
    // fn recieve_input(){}


    fn pair(stream: &mut TcpStream, password: &String) -> anyhow::Result<()> {
        // Hash the password into a `key_length` (32bit) key called `hash_key` (server)
        let password_key: String = sha1_crypt::hash_with(HashSetup {salt: Some("1"), rounds: ROUNDS, } , password)?;
        let password_key: &[u8] = password_key.as_bytes();

        // Create a random symmetrical key `sym_key` (server)
        let mut sym_key = [0u8; KEY_LEN];  OsRng.fill_bytes(&mut sym_key); 

        // Encrypt the `sym_key` using the `password_key` (server)
        let aead = XChaCha20Poly1305::new((&password_key[..KEY_LEN]).into());
        let mut nonce = [0u8; NONCE];  OsRng.fill_bytes(&mut nonce); 
    println!("Nonce: {:?}={}", nonce.len(), NONCE);

        let ciphertext = aead.encrypt(nonce[..].into(), sym_key.as_ref())
            .map_err(|e| anyhow!("sym_key encryption failed: {e}"))?;

        // Send the cyphertext (sym_key) to the client (syn-ack)
        let mut payload = Vec::new();   
        payload.push(data_type::PAIR);
        payload.extend_from_slice(&nonce[..]);
        payload.extend_from_slice(&ciphertext[..]);

        stream.write(&payload)
            .map_err(|e| anyhow!("Writing to stream: {e}"))?;

        Ok(())
    }


} // mod server
