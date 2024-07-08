pub mod client {

    use std::io::prelude::*;
    use std::net::TcpListener;
    use pwhash::{sha1_crypt, HashSetup};
    use chacha20poly1305::{
        aead::{Aead, KeyInit, OsRng},
        ChaCha20Poly1305,
        AeadCore,
    };
    use anyhow::Ok;


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

        let socket_addr = "localhost:3453"; //WORKS
        let listener = TcpListener::bind(socket_addr)?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut segment = Segment::new();
            let _read_count = stream.read(&mut segment.buffer)?;

            let ciphertext = segment.get_ciphertext();
            let nonce = segment.get_nonce();

            let plaintext = decrypt(&ciphertext, &nonce);

            println!("{}", String::from_utf8(plaintext?)?);
        }
        Ok(())
    }


    fn decrypt(ciphertext: &[u8], nonce: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = [0u8; KEY].as_ref().into();

        let cipher = ChaCha20Poly1305::new(key);

        let plaintext = cipher.decrypt(nonce.into(), &ciphertext[..]).expect("decrypts ciphertext");
        Ok(plaintext)
    }


    fn pair() -> Vec<u8> {
        let hash_setup = HashSetup {
            salt: Some("goodsalt"),
            rounds: Some(8),
        };

        sha1_crypt::hash_with(
            hash_setup,
            "password"
        ).unwrap();


        let sym_key = ChaCha20Poly1305::generate_key(&mut OsRng);

        // fetch the pw from the database
        let pw = b"very_strong_password";

        // hash pw into pw_key

        // encrypt key using pw_key
        // send encrypted key to client

        // use sym_key to receive segments
        sym_key.to_vec()
    }

} // mod client
