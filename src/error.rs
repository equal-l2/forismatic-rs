extern crate reqwest;
extern crate serde_json;
use std;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

pub fn too_long_key() -> Error {
    Error {
        kind: Kind::TooLongKey,
    }
}

#[derive(Debug)]
enum Kind {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    TooLongKey,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            Kind::Io(ref e) => write!(f, "Io Error: {}", e),
            Kind::Reqwest(ref e) => write!(f, "Reqwest Error: {}", e),
            Kind::SerdeJson(ref e) => write!(f, "Serde_json Error: {}", e),
            Kind::TooLongKey => write!(f, "Error: Key must not be longer than 6 chars"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::Io(ref e) => e.description(),
            Kind::Reqwest(ref e) => e.description(),
            Kind::SerdeJson(ref e) => e.description(),
            Kind::TooLongKey => "Key must not be longer than 6 chars",
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self {
            kind: Kind::Reqwest(e),
        }
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self {
            kind: Kind::SerdeJson(e),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Kind::Io(e) }
    }
}
