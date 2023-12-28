pub mod server {

    use anyhow::anyhow;
    use chacha20poly1305::{    
        aead::{stream, NewAead},                                                                                                                                                            
        XChaCha20Poly1305,    
    };    
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::fs::File;

    use crate::NONCE;
    // use std::path::Path;

    const BUFFER_SIZE: usize = 1024;
    // const NONCE: usize = 19;
    const PORT: u16 = 3453;

    // List of data transfer types
    enum DataType {
        File,
        Pair,
        Text,
        Invalid,
    }

    pub fn run_server() -> Result<(), anyhow::Error> {
        let key = [0u8; 32];
        let file_path = "output.txt";
        let ip_addr = String::from("localhost");

        decrypt_tcp(file_path, &key, &ip_addr)?;

        Ok(())
    }

    fn decrypt_tcp(
        output_path: &str,
        key: &[u8; 32],
        ip_addr: &String
    ) -> Result<(), anyhow::Error> {
        // create socket address
        let socket = format!("{}:{}", ip_addr, PORT);

        // create listener and bind it to the socket
        let listener = TcpListener::bind(socket).unwrap();
        let mut buffer = [0; BUFFER_SIZE];

        let aead = XChaCha20Poly1305::new(key.as_ref().into());

        // Use stream as source
        let mut output_file = File::create(output_path)?;

        // listen for incoming connections
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            // test to see how many times to loop iterates
            println!("looped");

            // Read in buffer, locate nonce
            let read_count = stream.read(&mut buffer).unwrap();
            let nonce = &buffer[..19];

            println!("nonce server: {:?}", nonce);
            
            let clone = aead.clone();
            // Initialize decryption variables 
            let mut stream_decryptor = stream::DecryptorBE32::from_aead(clone, nonce.as_ref().into());

            // Shows the number of bytes read
            println!("{}", read_count);

            // check if the data is a pair_request
            // if so, break the loop
            // data could also be a command, handle that appropriately

            if read_count == BUFFER_SIZE { 
                // If the buffer is full then expect more packets
                let plaintext = stream_decryptor
                    .decrypt_next(&buffer[NONCE..])
                    .map_err(|e| anyhow!("Decrypting large file step 1: {}", e))?;

                println!("data: {}", String::from_utf8_lossy(&plaintext));
                output_file.write(&plaintext)?;
            } else if read_count == 0 {
                // If there is no more data ... end
                println!("no data read");
                break;
            } else {
                println!("here");
                // If the buffer is neither empty nor full then this is the last packet
                let plaintext = stream_decryptor
                    .decrypt_last(&buffer[NONCE..read_count])
                    .map_err(|e| anyhow!("Decrypting large file step 2: {}", e))?;

                println!("data: {}", String::from_utf8_lossy(&plaintext));
                output_file.write(&plaintext)?;
                break;
            }

        }

        Ok(())
    }


    // Execute a command sent by the client
    fn recieve_cmd(buffer: &[u8; BUFFER_SIZE]) {}

    // Take in mouse and keyboard input from the client
    // Using UDP... ??
    fn recieve_input(){}


    fn pair_request(buffer: &[u8; BUFFER_SIZE]) {
        // this is where the key is shared
    }


} // mod server
