pub mod client {

    use std::io::prelude::*;
    use std::net::TcpStream;

    pub fn run_client () -> anyhow::Result<()> {
        let ip_addr = String::from("localhost:8081");
        let mut stream = TcpStream::connect(ip_addr)?;

        let message = String::from("Hello World @ UWI");
        stream.write(&message.as_bytes())?;
        Ok(())
    }
} // mod client
