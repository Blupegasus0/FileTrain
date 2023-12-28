pub mod client {

    use anyhow::anyhow;
    use chacha20poly1305::{    
        aead::{stream, Aead, AeadCore, NewAead},                                                                                                                                                            
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

    pub fn run_client() -> Result<(), anyhow::Error> {
        let key = [0u8; 32];
        let nonce = [0u8; 19];
        let file_path = "test.txt";
        let ip_addr = String::from("localhost");

        encrypt_tcp(file_path, &key, &ip_addr)?;

        Ok(())

    }

    fn encrypt_tcp(
        source_file_path: &str,
        key: &[u8; 32],
        ip_addr: &String,
    ) -> Result<(), anyhow::Error> {
        // create socket address
        let socket = format!("{}:{}", ip_addr, PORT);

        // Initialize encryption variables
        let aead = XChaCha20Poly1305::new(key.as_ref().into());
        let mut nonce = [0u8; 19];  OsRng.fill_bytes(&mut nonce);
            println!("nonce client: {:?}", nonce);
        let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

        let mut buffer = [0u8; BUFFER_SIZE];

        for i in 0..nonce.len() {
            // buffer[i] = nonce[i];
            buffer[i] = 0;
        }

        let mut source_file = File::open(source_file_path)?;

        loop {
            let read_count = source_file.read(&mut buffer)?;

            println!("{}", read_count);

            if read_count == BUFFER_SIZE {
                // If the buffer is full then expect more data
                let mut payload = Vec::new();   
                payload.extend_from_slice(&nonce[..]);
                let ciphertext = stream_encryptor
                    .encrypt_next(&buffer[..])
                    .map_err(|e| anyhow!("Encryping large file: {}", e))?;
                payload.extend_from_slice(&ciphertext[..]);

                // Connect to the stream
                let mut stream = TcpStream::connect(&socket).unwrap();
                //  Write message to the stream
                stream.write(&payload).unwrap();

            } else {
                // If the buffer is not full then send the ending packet
                let mut payload = Vec::new();   
                payload.extend_from_slice(&nonce[..]);
                let ciphertext = stream_encryptor
                    .encrypt_last(&buffer[..read_count])
                    .map_err(|e| anyhow!("Encryping large file: {}", e))?;
                payload.extend_from_slice(&ciphertext[..]);

                // Connect to the stream
                let mut stream = TcpStream::connect(&socket).unwrap();
                //  Write message to the stream
                stream.write(&payload).unwrap();

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
