extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::io::Read;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote{
    pub quote_text:   String,
    pub quote_author: String,
    pub sender_name:  String,
    pub sender_link:  String,
    pub quote_link:   String
}

pub enum Lang{
    EN,
    RU,
}

impl std::fmt::Display for Lang{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Lang::EN => write!(f,"en"),
            Lang::RU => write!(f,"ru")
        }
    }
}

#[derive(Debug)]
pub struct Error{
    kind : Kind
}

#[derive(Debug)]
pub enum Kind{
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

pub type Result<T> = std::result::Result<T,Error>;

pub fn get_quote<T>(lang: Lang, key: T) -> self::Result<Quote> where Option<u32>: From<T> {
    let url = match Option::from(key) {
        Some(k) => {
            if k.to_string().len() > 6 { return Err(self::Error{ kind : Kind::TooLongKey }); }
            format!("https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={}&key={}",lang,k)
        },
        None => format!("https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={}",lang)
    };
    let mut content = String::new();
    reqwest::get(&url)?.read_to_string(&mut content)?;
    let content = content.replace("\\'","'");
    let quote: Quote = serde_json::from_str(content.as_str())?;
    Ok(quote)
}
