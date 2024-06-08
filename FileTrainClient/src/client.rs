pub mod client {

    use std::io::prelude::*;
    use std::net::TcpListener;

    const NONCE: usize = 12;
    const KEY: usize = 32;
    const BUFFER: usize = 1024;


    pub fn run_client () -> anyhow::Result<()> {

        let ip_addr = "localhost:3453"; //WORKS

        let listener = TcpListener::bind(ip_addr)?;

        println!("read nothing");

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = [0u8; BUFFER];
            let read_count = stream.read(&mut buffer)?;
            
            let ciphertext_range = 0..read_count;
            let ciphertext: Vec<u8> = buffer[ciphertext_range].to_vec();

            let plaintext = decrypt(&ciphertext);

            println!("{}", String::from_utf8(plaintext?)?);
        }
        Ok(())
    }


    fn decrypt(payload: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            ChaCha20Poly1305, Nonce
        };


        let key = [0u8; KEY].as_ref().into();
        let nonce = &payload[..NONCE];
        // let nonce = [0u8; NONCE].as_ref().into();
        let cipher = ChaCha20Poly1305::new(key);

        let ciphertext = &payload[NONCE..];
        println!("{:?}",ciphertext);

        let plaintext = cipher.decrypt(nonce.into(), &ciphertext[..]).expect("decrypts ciphertext");
        Ok(plaintext)
    }
} // mod client
