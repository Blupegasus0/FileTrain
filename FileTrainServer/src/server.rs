pub mod server {
    use std::io::prelude::*;
    use std::net::TcpStream;
    use anyhow::Ok;

    const NONCE: usize = 12;
    const KEY: usize = 32;
    const PAYLOAD_LEN: usize = 2;


    pub fn run_server() -> anyhow::Result<()> {
        let ip_addr = String::from("localhost:3453");
        let mut stream = TcpStream::connect(ip_addr)?;

        // let message = String::from("Hello World @ UWI");
        let message = String::from("Different test");
        let ciphertext = encrypt(&message)?;

        stream.write(&ciphertext)?;
        stream.write(b"hello?")?;
        Ok(())
    }

    fn encrypt(plaintext: &String) -> anyhow::Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit, OsRng},
            ChaCha20Poly1305,
        };

        // let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let key = [0u8; KEY].as_ref().into();

        let cipher = ChaCha20Poly1305::new(key);

        // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let nonce = [0u8; NONCE].as_ref().into();

        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).expect("encrypts plaintext");

        let payload_len = (nonce.len() as u16 + ciphertext.len() as u16).to_be_bytes();

        // add nonce as header
        let mut payload = Vec::new(); payload.append(&mut payload_len.to_vec()); payload.append(&mut nonce.to_vec()); payload.append(&mut ciphertext.to_vec());

        Ok(payload)
    }

} // mod server
