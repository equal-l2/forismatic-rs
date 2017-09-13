extern crate reqwest;
extern crate serde_json;
use std;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    TooLongKey,
}

pub fn too_long_key() -> Error {
    Error::TooLongKey
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "Io Error: {}", e),
            Error::Reqwest(ref e) => write!(f, "Reqwest Error: {}", e),
            Error::SerdeJson(ref e) => write!(f, "Serde_json Error: {}", e),
            Error::TooLongKey => write!(f, "Error: Key must not be longer than 6 chars"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Reqwest(ref e) => e.description(),
            Error::SerdeJson(ref e) => e.description(),
            Error::TooLongKey => "Key must not be longer than 6 chars",
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
