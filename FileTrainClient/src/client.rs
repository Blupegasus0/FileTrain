pub mod client {

    use anyhow::{anyhow, Ok};
    use chacha20poly1305::{    
        XChaCha20Poly1305, aead::{stream, Aead, NewAead, Buffer}, 
    };    
    use rand::{RngCore, rngs::OsRng}; 
    use core::panic;
    use std::io::{Read, Write};
    use std::net::TcpStream;
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


    pub fn run_client() -> anyhow::Result<()> {
        // let file_path = "test_files/test.txt";
        let file_path = "/home/obsidian/WorkDocs/Micro Computers/2 - Loans and using Goal seek.pdf";
        let ip_addr = String::from("localhost");
        let password = String::from("password");

        encrypt_tcp(file_path, &ip_addr, &password)?;

        Ok(())

    }

    fn encrypt_tcp(
        source_file_path: &str,
        ip_addr: &String,
        password: &String
    ) -> anyhow::Result<()> {
        // create socket address and buffer
        let socket = format!("{}:{}", ip_addr, PORT);
        let mut buffer = [0u8; BUFFER_SIZE];

        let input_type = "pair"; // TEMPORARY - supposed to be read in from user
        let data_type = match input_type {
            "file" => data_type::FILE,
            "text" => data_type::TEXT,
            "pair" => {
                let _ = pair(&socket, &password)//.expect("pairing failed"); 
                    .map_err(|e| anyhow!("pairing: {e}"))?; // TEMPORARY
                return Ok(());
            }
            _ => panic!("Invalid input data type"),
        };

        let mut source_file = File::open(source_file_path)
            .map_err(|e| anyhow!("Cannot open source file: {e}"))?;
        // let mut stream = TcpStream::connect(&socket)?; // correct placement. single connection.

        loop {
            let aead = XChaCha20Poly1305::new(TEMP_KEY.as_ref().into());
            let mut nonce = [0u8; NONCE];  OsRng.fill_bytes(&mut nonce); 
            let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

            let read_count = source_file.read(&mut buffer[..])
                .map_err(|e| anyhow!("Reading source file: {e}"))?;

            // Add metadata headers
            let mut payload = Vec::new();   
            payload.push(data_type);
            payload.extend_from_slice(&nonce[..]);

            if read_count == BUFFER_SIZE {
                // If the buffer is full then expect more data
                let ciphertext = stream_encryptor
                    .encrypt_next(&buffer[..])
                    .map_err(|e| anyhow!("Encryping large file 1: {e}"))?;
                payload.extend_from_slice(&ciphertext[..]);

                //  Write message to the stream
                let mut stream = TcpStream::connect(&socket)
                    .map_err(|e| anyhow!("Connecting to server: {e}"))?; // INEFFICIENT - multiple connections
                
                stream.write(&payload)
                    .map_err(|e| anyhow!("Writing to stream: {e}"))?;

            } else {
                // If the buffer is not full then send the ending packet
                let ciphertext = stream_encryptor
                    .encrypt_last(&buffer[..read_count])
                    .map_err(|e| anyhow!("Encryping large file 2: {e}"))?;
                payload.extend_from_slice(&ciphertext[..]);

                //  Write message to the stream
                let mut stream = TcpStream::connect(&socket)
                    .map_err(|e| anyhow!("Connecting to server: {e}"))?; // INEFFICIENT - multiple connections

                stream.write(&payload)
                    .map_err(|e| anyhow!("Writing to stream: {e}"))?;

                break;
            }
        }

        Ok(())
    }

    // handle commands

    // handle udp input


    fn pair(socket: &String, password: &String) -> anyhow::Result<()> {
        // Add metadata headers
        let mut payload = Vec::new();   
        payload.push(data_type::PAIR);
        
        // Send a signal to the server that {device name} is trying to connect.
        let mut stream = TcpStream::connect(&socket)
            .map_err(|e| anyhow!("Connecting to server: {e}"))?; 
        stream.write(&payload)
            .map_err(|e| anyhow!("Writing to stream: {e}"))?;

        // Hash the password into a `key_length` (32bit) key called `password_key`
        let password_key: String = sha1_crypt::hash_with(HashSetup {salt: Some("1"), rounds: ROUNDS, } , password)?;
        let password_key: &[u8] = password_key.as_bytes();

        // Receive sym_key
        let mut stream = TcpStream::connect(&socket)
            .map_err(|e| anyhow!("Connecting to server: {e}"))?; 
        let mut buffer = [0u8; METADATA+KEY_LEN];
        let _read_count = stream.read(&mut buffer)?;
        let nonce_start = METADATA - NONCE; let nonce_end = nonce_start + NONCE;
        let nonce = &buffer[nonce_start..nonce_end];

        let ciphertext = &buffer[METADATA..KEY_LEN+METADATA];
        
        // Decrypt sym_key
        let aead = XChaCha20Poly1305::new((&password_key[..KEY_LEN]).into());
        let sym_key = aead.decrypt(nonce.as_ref().into(), ciphertext.as_ref())
            .map_err(|e| anyhow!("decryption failed: {e}"))?;

        println!("sym_key: {:?}", String::from_utf8_lossy(&sym_key));


        Ok(())

    }

} // mod client
