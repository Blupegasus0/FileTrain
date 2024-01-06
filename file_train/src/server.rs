pub mod server {

    use anyhow::{anyhow, Ok};
    use chacha20poly1305::{    
        aead::{stream, NewAead},                                                                                                                                                            
        XChaCha20Poly1305,    
    };    
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::fs::File;
    // use std::path::Path;

    use crate::METADATA;
    use crate::NONCE;
    use crate::BUFFER_SIZE;
    use crate::PORT;
    use crate::TEMP_KEY;
    use crate::data_type;
    // use crate::DataType;

    const MISC_HEADERS: usize = 16;


    pub fn run_server() -> anyhow::Result<()> {
        // let file_path = "output.txt";
        let file_path = "output.pdf";
        let ip_addr = String::from("localhost");

        decrypt_tcp(file_path, &ip_addr)?;

        Ok(())
    }


    fn decrypt_tcp(
        output_path: &str,
        ip_addr: &String
    ) -> anyhow::Result<()> {

        let socket = format!("{}:{}", ip_addr, PORT);

        // create listener and bind it to the socket
        let listener = TcpListener::bind(socket)
            .expect("Failed to bind to socket");
        let mut buffer = [0; BUFFER_SIZE+METADATA+MISC_HEADERS];

        let mut output_file = File::create(output_path)
            .map_err(|e| anyhow!("Creating output file: {e}"))?;

        // listen for incoming connections
        for stream in listener.incoming() {

            let mut stream = stream?;

            // Read in buffer, locate nonce
            let read_count = stream.read(&mut buffer)?;
            let data_type = &buffer[0..1];
            let nonce_start = METADATA - NONCE; let nonce_end = nonce_start + NONCE;
            let nonce = &buffer[nonce_start..nonce_end];

            // Initialize decryption variables 
            let aead = XChaCha20Poly1305::new(TEMP_KEY[..].into());
            let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.into());

            // if so, break the loop
            // data could also be a command, handle that appropriately

            if read_count == buffer.len() { 
                // If the buffer is full then expect more packets
                let plaintext = stream_decryptor
                    .decrypt_next(&buffer[METADATA..])
                    .map_err(|e| anyhow!("Decrypting large file step 1: {}", e))?;
                
            // check if the data is a pair_request
            match data_type {
                // data_type::FILE => {},
                _ => break,
            }

                output_file.write(&plaintext)?;
            } else if read_count == 0 {
                // If there is no more data ... end
                println!("no data read");
                break;
            } else {
                // If the buffer is neither empty nor full then this is the last packet
                let plaintext = stream_decryptor
                    .decrypt_last(&buffer[METADATA..read_count])
                    .map_err(|e| anyhow!("Decrypting large file step 2: {:?}", e))?;

                output_file.write(&plaintext)?;
                break;
            }

        }

        Ok(())
    }

    fn handle_plaintext(data_type: &[u8; 1], buffer: Vec<u8> ) {

    }

    // Execute a command sent by the client
    fn recieve_cmd(buffer: &[u8; BUFFER_SIZE]) {}

    // Take in mouse and keyboard input from the client
    // Using UDP... ??
    fn recieve_input(){}


    fn pair(buffer: &[u8; BUFFER_SIZE]) {
        // this is where the key is shared
    }


} // mod server
