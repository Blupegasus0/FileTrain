pub mod server {

    use std::io::prelude::*;
    use std::net::TcpStream;

    use anyhow::Ok;


    pub fn run_server() -> anyhow::Result<()> {
        let ip_addr = String::from("localhost:8081");
        let mut stream = TcpStream::connect(ip_addr)?;

        let message = String::from("Hello World @ UWI");
        let ciphertext = encrypt(&message)?;

        stream.write(&ciphertext)?;
        Ok(())
    }

    fn encrypt(plaintext: &String) -> anyhow::Result<Vec<u8>> {

        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            ChaCha20Poly1305,
        };

        let key = ChaCha20Poly1305::generate_key(&mut OsRng);

        let cipher = ChaCha20Poly1305::new(&key);
        // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let nonce = [0u8; 12];

        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).expect("encrypts plaintext");

        Ok(ciphertext)
    }

} // mod server
