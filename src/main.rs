//! Runs TCP client and read line by line
//!
//! # Example
//! ```
//! cargo run
//! ```

use std::net::TcpStream;
use std::io::{BufReader, BufRead};

const PORT: &str = "9992";

/// Entry point
fn main() {
    match TcpStream::connect("127.0.0.1:".to_owned() + PORT) {
        Ok(stream) => {
            println!("Successfully connected to server in port {}.", PORT);
            let mut reader = BufReader::new(stream);
            loop {
                let mut data = String::new();
                let result = reader.read_line(&mut data);
                match result {
                    Ok(_) => {
                        if String::is_empty(&data) {
                            println!("Data is empty");
                            break;
                        }
                        println!("Data reply: {:?}", data);
                    }
                    Err(e) => {
                        println!("Data error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Connection is terminated.");
}
