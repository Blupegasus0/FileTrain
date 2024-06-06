pub mod client;

use crate::client::client::run_client;


fn main() {
        let _client = match run_client() {
            Err(e) => println!("Error: {e}"), // Display error to user
            _ => {},
        };

}



