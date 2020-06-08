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

#[derive(Debug)]
enum ButtonState {
    Released = 0,
    Pressed = 1,
    Between = 2,
}

/// Extension for Capture
trait GetButtonValueFromCaptures {
    fn get_index(&self) -> u8;
    fn get_state(&self) -> ButtonState;
}

impl GetButtonValueFromCaptures for Captures<'_> {
    fn get_index(&self) -> u8 {
        return self.get(1).unwrap().as_str().parse::<u8>().unwrap();
    }
    fn get_state(&self) -> ButtonState {
        match self.get(2).unwrap().as_str().parse::<u8>().unwrap() {
            0 => ButtonState::Released,
            1 => ButtonState::Pressed,
            2 | _ => ButtonState::Between,
        }
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
                                let index = captures.get_index();
                                let state = captures.get_state();
                                println!("Package index {}, state: {:?}", index, state);
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
