use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub ipaddr: String,
    pub password: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramInfo {
    pub author: String,
    pub description: String,
    pub version: String,
    pub project: String,
    pub help_post_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_name: String,
}

