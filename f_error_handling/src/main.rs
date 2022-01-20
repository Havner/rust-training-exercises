// Implement 'call' function with:
// - 'check_format' function and return error when necessary,
// - 'call' function and pass value or error accordingly
// Add information messages for given situations (remove panic! behavior):
// - when connection fails because the number is in wrong format
// - when number could not be reached
// - when connection was successfully opened
// - when something unexpected happened

use regex::Regex;
use std::io;

struct Connection {
    pub number: String,
}

#[derive(Debug)]
enum ConnectionError {
    WrongNumberFormat,
    FormatterError,
}

// expected format: xxx-xxx-xxx
fn check_format(number: &str) -> Result<(), ConnectionError> {
    let re = match Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{3}") {
        Ok(re) => re,
        Err(_) => return Err(ConnectionError::FormatterError),
    };

    match re.is_match(number) {
        true => Ok(()),
        false => Err(ConnectionError::WrongNumberFormat),
    }
}

fn connect(number: &str) -> Result<Option<Connection>, ConnectionError> {
    if number.chars().nth(8).unwrap() > '5' {
        return Ok(None);
    }

    Ok(Some(Connection { number: number.to_string() }))
}

fn call(number: &str) -> Result<Option<Connection>, ConnectionError> {
    // implement
    Err(ConnectionError::WrongNumberFormat)
}

fn main() {
    println!("What number do you want to call?");
    let mut number = String::new();
    // change behavior from panic! to println! + return
    io::stdin().read_line(&mut number).expect("Failed to read from stdin!");
    call(number.trim()).expect("Failed to call number");
}
