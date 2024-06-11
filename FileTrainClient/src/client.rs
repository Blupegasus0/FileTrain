pub mod client {

    use std::io::prelude::*;
    use std::net::TcpListener;

    const NONCE: usize = 12;
    const KEY: usize = 32;
    const BUFFER: usize = 1024;
    const PAYLOAD_LEN: usize = 2;


    // Create a struct for the segment/payload that does the parsing intrinsically to tidy up
    // decryption function.
    struct Segment {
        buffer: [u8; BUFFER],
    }

    impl Segment {
        fn new() -> Segment {
            Segment {
                buffer: [0u8; BUFFER]
            }
        }

        fn get_payload_len(&self) -> usize {
            u16::from_be_bytes([self.buffer[PAYLOAD_LEN-2], self.buffer[PAYLOAD_LEN-1]]) as usize
        }
        fn get_ciphertext(&self) -> &[u8] {
            let payload_len = u16::from_be_bytes([self.buffer[PAYLOAD_LEN-2], self.buffer[PAYLOAD_LEN-1]]) as usize;
            &self.buffer[PAYLOAD_LEN+NONCE..PAYLOAD_LEN+payload_len]
        }
        fn get_nonce(&self) -> &[u8] {
            &self.buffer[PAYLOAD_LEN..PAYLOAD_LEN+NONCE]
        }
    }


    pub fn run_client () -> anyhow::Result<()> {

        let ip_addr = "localhost:3453"; //WORKS

        let listener = TcpListener::bind(ip_addr)?;

        println!("read nothing");

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut segment = Segment::new();
            let read_count = stream.read(&mut segment.buffer)?;
            
            let ciphertext = segment.get_ciphertext();
            let nonce = segment.get_nonce();

            let plaintext = decrypt(&ciphertext, &nonce);

            println!("{}", String::from_utf8(plaintext?)?);
        }
        Ok(())
    }


    fn decrypt(ciphertext: &[u8], nonce: &[u8]) -> anyhow::Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            ChaCha20Poly1305, Nonce
        };

        let key = [0u8; KEY].as_ref().into();
        // let nonce = &payload[PAYLOAD_LEN..PAYLOAD_LEN+NONCE];

        let cipher = ChaCha20Poly1305::new(key);

        let plaintext = cipher.decrypt(nonce.into(), &ciphertext[..]).expect("decrypts ciphertext");
        Ok(plaintext)
    }

    fn decrypt_old(payload: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            ChaCha20Poly1305, Nonce
        };


        let key = [0u8; KEY].as_ref().into();
        let nonce = &payload[PAYLOAD_LEN..PAYLOAD_LEN+NONCE];

        let cipher = ChaCha20Poly1305::new(key);

        let payload_len = u16::from_be_bytes([payload[PAYLOAD_LEN-2], payload[PAYLOAD_LEN-1]]) as usize;
        println!("{:?}", &payload[..PAYLOAD_LEN]);
        println!("{:?}",payload_len);

        // let ciphertext = &payload[PAYLOAD_LEN+NONCE..];
        let ciphertext = &payload[PAYLOAD_LEN+NONCE..PAYLOAD_LEN+payload_len];
        println!("{:?}",ciphertext);

        let plaintext = cipher.decrypt(nonce.into(), &ciphertext[..]).expect("decrypts ciphertext");
        Ok(plaintext)
    }
} // mod client
