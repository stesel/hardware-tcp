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

#[derive(Debug)]
enum ButtonState {
    Released = 0,
    Pressed = 1,
    Between = 2,
}

/// Extension for Capture
trait ButtonValuable {
    fn new(raw:&str) -> Self;
}

struct ButtonPackage {
    index: u8,
    state: ButtonState,
}

impl ButtonValuable for ButtonPackage {
    fn new(raw: &str) -> ButtonPackage {
        let trimmed = raw
            .replace("{", "")
            .replace("}", "")
            .replace("\r\n", "");

        let vec: Vec<&str> = trimmed.split(",").collect::<Vec<&str>>();

        let index: u8 = match vec.get(0).unwrap().parse::<u8>() {
            Ok(index) => index,
            Err(_) => 0,
        };

        let state: ButtonState = match vec.get(1) {
            Some(&"1") => ButtonState::Pressed,
            Some(&"2") => ButtonState::Between,
            Some(&"0") | _ => ButtonState::Released,
        };

        return ButtonPackage {
            index,
            state,
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

                        let package: ButtonPackage = ButtonPackage::new(&data);
                        println!("ButtonPackage: {} {:?}", package.index, package.state);
                    }
                    Err(e) => {
                        println!("Data error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {:?}", e);
        }
    }

    println!("Connection is terminated.");
}
