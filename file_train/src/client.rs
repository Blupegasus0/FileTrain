pub mod client {

    use anyhow::anyhow;
    use chacha20poly1305::{    
        XChaCha20Poly1305, aead::{stream, NewAead}, 
    };    
    use rand::{RngCore, rngs::OsRng}; 
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::fs::File;
    // use std::path::Path;
    
    use crate::NONCE;
    use crate::BUFFER_SIZE;
    use crate::PORT;
    use crate::TEMP_KEY;
    use crate::data_type;
    // use crate::DataType;


    pub fn run_client() -> anyhow::Result<()> {
        // let file_path = "test.txt";
        let file_path = "/home/obsidian/WorkDocs/Micro Computers/2 - Loans and using Goal seek.pdf";
        let ip_addr = String::from("localhost");

        encrypt_tcp(file_path, &ip_addr)?;

        Ok(())

    }

    fn encrypt_tcp(
        source_file_path: &str,
        ip_addr: &String,
    ) -> anyhow::Result<()> {
        // create socket address
        let socket = format!("{}:{}", ip_addr, PORT);

        let mut buffer = [0u8; BUFFER_SIZE];

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
            payload.extend_from_slice(&data_type::FILE[..]);
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

} // mod client
