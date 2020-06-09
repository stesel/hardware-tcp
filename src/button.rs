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
#[derive(PartialEq)]
enum ButtonState {
    Released = 0,
    Pressed = 1,
    Between = 2,
}

/// Extension for button package
pub trait ButtonValuable {
    fn new(raw:&str) -> Self;
}

pub struct ButtonPackage {
    index: u8,
    state: ButtonState,
}

impl ButtonValuable for ButtonPackage {
    fn new(raw: &str) -> ButtonPackage {
        let trimmed = raw
            .replace("\r\n", "")
            .replace("{", "")
            .replace("}", "");

        let vec: Vec<&str> = trimmed.split(",").collect::<Vec<&str>>();

        let index: u8 = match vec.get(0).unwrap().parse::<u8>() {
            Ok(index) => index,
            Err(_) => 0,
        };

        let state: ButtonState = match vec.get(1) {
            Some(&"1") | Some(&"01") => ButtonState::Pressed,
            Some(&"2") | Some(&"02") => ButtonState::Between,
            _ => ButtonState::Released,
        };

        return ButtonPackage {
            index,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_data() {
        let package = ButtonPackage::new("");
        assert_eq!(0, package.index);
        assert_eq!(ButtonState::Released, package.state);
    }

    #[test]
    fn wrong_data() {
        let package = ButtonPackage::new("Welcome");
        assert_eq!(0, package.index);
        assert_eq!(ButtonState::Released, package.state);
    }

    #[test]
    fn expected_data() {
        let package = ButtonPackage::new("{8,1}\r\n");
        assert_eq!(8, package.index);
        assert_eq!(ButtonState::Pressed, package.state);
    }

    #[test]
    fn o_start_data() {
        let package = ButtonPackage::new("{09,02}\r\n");
        assert_eq!(9, package.index);
        assert_eq!(ButtonState::Between, package.state);
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
