//! Runs TCP button client and read state line by line
//!
//! # Example
//! ```
//! mod button;
//! button::create_connection("9999");
//! ```
//!
use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use regex::{Regex, Captures};

/// Extension for Capture
trait GetButtonValueFromCaptures<T> {
    fn get_value(&self, index: usize) -> T;
}

impl GetButtonValueFromCaptures<u8> for Captures<'_> {
    fn get_value(&self, index: usize) -> u8 {
        return self.get(index).unwrap().as_str().parse::<u8>().unwrap()
    }
}
///

/// Create button client connection
pub fn create_connection(port: &str) {
    match TcpStream::connect("127.0.0.1:".to_owned() + port) {
        Ok(stream) => {
            println!("Successfully connected to server in port {}.", port);
            let mut reader = BufReader::new(stream);
            loop {
                let mut data = String::new();
                match reader.read_line(&mut data) {
                    Ok(result) => {
                        if result == 0 {
                            println!("Data is closed.");
                            break;
                        }

                        let pattern = Regex::new(r"\{(\d+),\s*(\d+)}\r\n").unwrap();
                        match pattern.captures(&data) {
                            Some(captures) => {
                                let index = captures.get_value(1);
                                let state = captures.get_value(2);
                                println!("Package index {}, state: {}", index, state);
                            },
                            None => println!("Irrelevant data: {:?}", data),
                        }
                    }
                    Err(e) => {
                        println!("Data error: {:?}", e);
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
