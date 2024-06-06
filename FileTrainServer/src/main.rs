pub mod server;

use crate::server::server::run_server;


fn main() {
    let _server = match run_server() {
        Err(e) => println!("Error: {e}"), // Display error to user
        _ => {},
    };
}



