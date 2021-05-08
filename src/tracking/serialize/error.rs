use serde::ser;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(String);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.0)
    }
}
impl std::error::Error for Error {}
impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(format!("{}", msg))
    }
}
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error(String::from(err))
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Error(err)
    }
}
