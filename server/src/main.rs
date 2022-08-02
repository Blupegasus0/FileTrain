// Server

use anyhow::anyhow;
use chacha20poly1305::{    
    aead::{stream, Aead, NewAead},                                                                                                                                                            
    XChaCha20Poly1305,    
};    
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::Path;

const BUFFER_SIZE: usize = 1024;
const PORT: u16 = 3453;

fn main() {
    // create listener and bind it to localhost port 7878
    let listener = TcpListener::bind("localhost:3453").unwrap();

    // listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// List of data transfer types
enum data_type {
    file,
    pair,
    text,
    invalid,
}

fn handle_connection(mut stream: TcpStream) {
    // Read in buffer and handle any errors
    let mut buffer = [0; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    // Decrypt data ?

    // convert data to a string
    //DISABLED
    //let data_str = String::from_utf8((&mut buffer).to_vec()).unwrap();

    // Recieve metadata so this function knows what type of data its is recieving
    // and use that data to choose which function to call.

    // Define possible data prefixes
    let file = String::from("FILE");
    let text = String::from("TEXT");
    let pair = String::from("PAIR");    
    // PREFIX DISABLED
    let prefix = file.clone();//String::from_utf8((&buffer[..4]).to_vec()).unwrap();


    // Assign the data prefix to the appropriate enum type
    let data_type;
    let bytes_to_remove: u8;
    if &prefix == &file {
        data_type = data_type::file;
        //bytes to remove = 4+10+64; // (data type + file size + file name)

    } else if &prefix == &text {
        data_type = data_type::text;
        //bytes to remove = 4+10; // (data type + file size )

    } else if &prefix == &pair {
        data_type = data_type::pair;

    } else {
        data_type = data_type::invalid;
    }
    println!("{}", prefix);


    // After the metadata is read, remove it appropriately and pass it to the 
    // appropriate function.

    // Remove the prefix (metadata) from the data


    // Pass the data to the appropriate function
    match data_type {
        ///// FOR TESTING ONLY
        data_type::file => recieve(&buffer),
        data_type::pair => recieve(&buffer),
        data_type::text => recieve(&buffer),
        data_type::invalid => recieve(&buffer),
        //data_type::file => recieve_file(&buffer),
        //data_type::pair => pair_request(&buffer),
        //data_type::text => recieve_text(&buffer),
        //data_type::invalid => panic!("Invalid prefix"),
    }
}

fn recieve_file(&buffer: &[u8; BUFFER_SIZE]) {
    // If the data is the first packet then:
        // Create new file and give it the name recieved
        let file_name = "recieved.txt";

        let mut file = fs::File::options()
        .append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // Append the buffer to the file
    file.write(&buffer);
    //file.write(b"appended");
    println!("new buffer");
    
}


//// DECRYPTION TEST FUNCTION 
fn recieve(buffer: &[u8; BUFFER_SIZE]) {
    // Parameters: 
    // encrypted_file_path  
    // output_path
    // key
    // nonce

    let output_path = "long_decrypt.txt";

    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];

    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = buffer;
    //let mut buffer = [0u8; BUFFER_LEN];


    //let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut output_file = fs::File::create(output_path);

    loop {
        //let read_count = encrypted_file.read(&mut buffer);
        let read_count = buffer.len();

        if read_count == BUFFER_LEN { 
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|e| anyhow!("Decrypting large file: {}", e));

            println!("{}", String::from_utf8_lossy(&plaintext.unwrap()));
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|e| anyhow!("Decrypting large: {}", e));

            println!("{}", String::from_utf8_lossy(&plaintext.unwrap()));
            break;
        }

    }

    // recieve and print text
    println!("Message: {}", String::from_utf8_lossy(&buffer[..]));
}

fn recieve_text(buffer: &[u8; BUFFER_SIZE]) {
    // recieve and print text
    println!("Message: {}", String::from_utf8_lossy(&buffer[..]));
}

// Exicute a command sent by the client
fn recieve_cmd(buffer: &[u8; BUFFER_SIZE]) {}

// Take in mouse and keyboard input from the client
// Using UDP... ??
fn recieve_input(){}


fn pair_request(buffer: &[u8; BUFFER_SIZE]) {
    // this is where the key and nonce are shared
}