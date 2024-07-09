pub mod server {

    use std::io::prelude::*;
    use std::net::TcpStream;
    use pwhash::{sha1_crypt, HashSetup};
    use chacha20poly1305::{
        aead::{Aead, KeyInit, OsRng},
        ChaCha20Poly1305,
        AeadCore,
    };
    use anyhow::Ok;
    use cliparser::{help, parse, version};
    use toml;
    use std::collections::{HashMap, HashSet};
    use std::{env, process, fs};
    
    use crate::data_models::{Config, Database, ProgramInfo};

    const NONCE: usize = 12;
    const KEY: usize = 32;
    const BUFFER: usize = 1024;
    const PAYLOAD_LEN: usize = 2;
    const PORT: usize = 3453;

    pub fn run_server(arg_map: HashMap<String, Vec<String>>) -> anyhow::Result<()> {
        let ipaddr = &arg_map.get("ip address").unwrap()[0];
        let socket_addr = format!("{}:{}",ipaddr,PORT);
        // let socket_addr = String::from("localhost:3453");
        let mut stream = TcpStream::connect(socket_addr)?;

        let message = &arg_map.get("message").unwrap()[0];
        // let message = String::from("Hello World @ UWI");
        let ciphertext = encrypt(&message)?;

        // println!("arg_map: {:?}", arg_map.get("message"));
        stream.write(&ciphertext)?;
        Ok(())
    }

    fn encrypt(plaintext: &String) -> anyhow::Result<Vec<u8>> {
        let key = [0u8; KEY].into();
        let cipher = ChaCha20Poly1305::new(&key);

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).expect("encrypts plaintext");

        let payload_len = (nonce.len() as u16 + ciphertext.len() as u16).to_be_bytes();

        // add nonce as header
        let mut payload = Vec::new(); payload.append(&mut payload_len.to_vec()); payload.append(&mut nonce.to_vec()); payload.append(&mut ciphertext.to_vec());

        Ok(payload)
    }


    fn pair() -> Vec<u8> {

        sha1_crypt::hash_with(
            "$sha1$19703$iVdJqfSE$v4qYKl1zqYThwpjJAoKX6UvlHq/a",
            "password"
        ).unwrap();


        // fetch the pw from the database
        let pw = b"very_strong_password";

        // send pair request segment (syn)

        // hash pw to create pw_key
        // decrypt sym_key using pw_key

        // use sym_key to send segments
        let sym_key: [u8; KEY] = [0u8; KEY].into();
        sym_key.to_vec()

    }



    #[test]
    fn hash_this_mf() {
        let hash_setup = HashSetup {
            salt: Some("goodsalt"),
            rounds: Some(8),
        };

        let my_lil_key = sha1_crypt::hash_with(
            hash_setup,
            "password"
        ).unwrap();

        println!("{:?}", my_lil_key);
    }


} // mod server


