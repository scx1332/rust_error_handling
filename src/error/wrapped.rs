use std::error::Error;
use super::ErrorBag;

/// Error type build over ErrorBag, containing source code location and optional message
/// Note that only creating via macro is possible to catch line and file
#[derive(Debug)]
pub struct WrappedError {
    pub inner: ErrorBag,
    pub msg: Option<String>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl std::fmt::Display for WrappedError {
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
