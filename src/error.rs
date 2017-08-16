extern crate reqwest;
extern crate serde_json;
use std;
#[derive(Debug)]
pub struct Error{
    kind : Kind
}

pub fn too_long_key() -> Error {
    Error { kind : Kind::TooLongKey }
}

#[derive(Debug)]
enum Kind{
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
    TooLongKey,
}

impl std::convert::From<reqwest::Error> for self::Error {
    fn from(e: reqwest::Error) -> Self {
        Self{ kind : Kind::Reqwest(e) }
    }
}

impl std::convert::From<serde_json::Error> for self::Error {
    fn from(e: serde_json::Error) -> Self {
        Self{ kind : Kind::SerdeJson(e) }
    }
}

impl std::convert::From<std::io::Error> for self::Error {
    fn from(e: std::io::Error) -> Self {
        Self{ kind : Kind::Io(e) }
    }
}
