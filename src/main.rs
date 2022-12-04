mod error;

use crate::error::{LibError, LibErrorEnum};
use std::fs::File;
use std::io::Read;

fn read_number_from_file_simple_errors(filename: &str) -> Result<u64, LibErrorEnum> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let parsed: u64 = buffer.trim().parse()?;

    Ok(parsed)
}

fn read_number_from_file_wrapped_errors(filename: &str) -> Result<u64, LibError> {
    let mut file = File::open(filename).map_err(errmsg!("Cannot open {}", filename))?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).map_err(err!())?;

    let parsed: u64 = buffer
        .trim()
        .parse()
        .map_err(errmsg!("Error when parsing {}", filename))?;

    Ok(parsed)
}

fn main() {
    match read_number_from_file_simple_errors("number.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_number_from_file_simple_errors("src/error.rs") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_number_from_file_wrapped_errors("number.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_number_from_file_wrapped_errors("src/error.rs") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}
