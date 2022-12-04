use rust_error_handling::{
    err_create, err_from, err_from_msg, CustomError, ErrorBag, WrappedError,
};
use std::fs::File;
use std::io::Read;

fn read_num_simple(filename: &str) -> Result<u64, ErrorBag> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let parsed: u64 = buffer.trim().parse()?;
    if parsed > 100 {
        return Err(ErrorBag::CustomError(CustomError::new("Number is too big")));
    }
    //... do something with the number
    Ok(parsed)
}

fn read_num_wrapped(filename: &str) -> Result<u64, WrappedError> {
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
    match read_num_simple("examples/input/not_exists.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("examples/input/number_invalid.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("examples/input/number_1000.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_simple("examples/input/number_10.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }

    match read_num_wrapped("examples/input/not_exists.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("examples/input/number_invalid.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("examples/input/number_1000.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
    match read_num_wrapped("examples/input/number_10.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}
