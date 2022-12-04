use std::error::Error;
use std::fmt::Display;

use std::num::ParseIntError;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(message: &str) -> CustomError {
        CustomError {
            message: message.to_string(),
        }
    }
}
impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

#[derive(Debug)]
pub enum LibErrorEnum {
    ParseError(ParseIntError),
    IoError(std::io::Error),
    CustomError(CustomError),
}

// New error type encapsulating the original error and location data.
#[derive(Debug)]
pub struct LibError {
    pub inner: LibErrorEnum,
    pub msg: Option<String>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl Error for LibError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.msg {
            write!(
                f,
                "{}, {}, {}:{}:{}",
                msg, self.inner, self.file, self.line, self.column
            )
        } else {
            write!(
                f,
                "{}, {}:{}:{}",
                self.inner, self.file, self.line, self.column
            )
        }
    }
}

#[macro_export]
macro_rules! err_create {
    ($t:expr) => {
        LibError {
            inner: LibErrorEnum::from($t),
            msg: None,
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}


// Macro to catch file name
#[macro_export]
macro_rules! err_from {
    () => {
        |e| LibError {
            inner: LibErrorEnum::from(e),
            msg: None,
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

#[macro_export]
macro_rules! err_from_msg {
    ($($t:tt)*) => {{
        |e| LibError {
            inner: LibErrorEnum::from(e),
            msg: Some(format!($($t)*)),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }};
}

impl Display for LibErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibErrorEnum::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
            LibErrorEnum::IoError(io_error) => write!(f, "{}", io_error),
            LibErrorEnum::CustomError(custom_error) => write!(f, "{}", custom_error),
        }
    }
}

impl std::error::Error for LibErrorEnum {}

impl From<ParseIntError> for LibErrorEnum {
    fn from(err: ParseIntError) -> Self {
        LibErrorEnum::ParseError(err)
    }
}

impl From<std::io::Error> for LibErrorEnum {
    fn from(err: std::io::Error) -> Self {
        LibErrorEnum::IoError(err)
    }
}

impl From<CustomError> for LibErrorEnum {
    fn from(err: CustomError) -> Self {
        LibErrorEnum::CustomError(err)
    }
}
