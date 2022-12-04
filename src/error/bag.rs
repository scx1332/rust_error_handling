use std::fmt::Display;
use std::num::ParseIntError;
use super::CustomError;

/// Enum containing all possible errors used in the library
/// Probably you can use thiserror crate to simplify this process
#[derive(Debug)]
pub enum ErrorBag {
    ParseError(ParseIntError),
    IoError(std::io::Error),
    CustomError(CustomError),
}


impl Display for ErrorBag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorBag::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
            ErrorBag::IoError(io_error) => write!(f, "{}", io_error),
            ErrorBag::CustomError(custom_error) => write!(f, "{}", custom_error),
        }
    }
}

impl std::error::Error for ErrorBag {}

impl From<ParseIntError> for ErrorBag {
    fn from(err: ParseIntError) -> Self {
        ErrorBag::ParseError(err)
    }
}

impl From<std::io::Error> for ErrorBag {
    fn from(err: std::io::Error) -> Self {
        ErrorBag::IoError(err)
    }
}

impl From<CustomError> for ErrorBag {
    fn from(err: CustomError) -> Self {
        ErrorBag::CustomError(err)
    }
}
