mod error;

use std::fmt::format;
use crate::error::LibError;
use crate::error::LibErrorEnum;

use std::fs::File;
use std::io::Read;


fn read_number_from_file(filename: &str) -> Result<u64, LibError> {
    let mut file = File::open(filename).map_err(errmsg!("Cannot open {}", filename))?; // Error!

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).map_err(err!())?; // Error

    let parsed: u64 = buffer.trim().parse().map_err(errmsg!("Error when parsing {}", filename))?; // Error

    Ok(parsed)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_number_from_file("src/error.rs") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}