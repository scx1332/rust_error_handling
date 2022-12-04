mod error;

use crate::error::{CustomError, LibError, LibErrorEnum};
use std::fs::File;

use std::io::Read;

fn read_num_simple(filename: &str) -> Result<u64, LibErrorEnum> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let parsed: u64 = buffer.trim().parse()?;
    if parsed > 100 {
        return Err(LibErrorEnum::CustomError(CustomError::new("Number is too big")));
    }
    //... do something with the number
    Ok(parsed)
}

fn read_num_wrapped(filename: &str) -> Result<u64, LibError> {
    let mut file = File::open(filename).map_err(err_from_msg!("Cannot open {}", filename))?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).map_err(err_from!())?;

    let parsed: u64 = buffer
        .trim()
        .parse()
        .map_err(err_from_msg!("Error when parsing {}", filename))?;

    if parsed > 100 {
        return Err(err_create!(CustomError::new("Number is too big")));
    }
    //... do something with the number
    Ok(parsed)
}

fn main() {
    match read_num_simple("not_exists.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("number_invalid.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("number_1000.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("number_10.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }

    match read_num_wrapped("not_exists.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("number_invalid.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("number_1000.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("number_10.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}
