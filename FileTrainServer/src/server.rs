pub mod server {

    use std::io::prelude::*;
    use std::net::TcpListener;


    pub fn run_server() -> anyhow::Result<()> {
        let ip_addr = "localhost:8081"; //WORKS

        let listener = TcpListener::bind(ip_addr)?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = [0u8; 1024];
            stream.read(&mut buffer)?;
            println!("{}", String::from_utf8_lossy(&buffer));
        }
        Ok(())
    }

} // mod serveruse std::io::prelude::*;

#[test]
fn encrypt() {
    use sodiumoxide::crypto::secretbox;
    
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();

    let plaintext = b"some data";

    let ciphertext = secretbox::seal(plaintext, &nonce, &key);

    let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();

    assert!(plaintext == &their_plaintext[..]);
}

