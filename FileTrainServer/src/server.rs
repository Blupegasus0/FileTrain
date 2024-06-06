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

