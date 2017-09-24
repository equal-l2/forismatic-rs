extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
pub mod error;
use std::io::Read;
use error::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub quote_text: String,
    pub quote_author: String,
    pub sender_name: String,
    pub sender_link: String,
    pub quote_link: String,
}

/// Language specification.
#[derive(Clone)]
pub enum Lang {
    EN,
    RU,
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Lang::EN => write!(f, "en"),
            Lang::RU => write!(f, "ru"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Get quote via forismatic API.
///
/// `key` must not be longer than 6 characters in string form.
/// # Examples
/// To get quote in English without key :
///
/// ```
/// get_quote(Lang::EN, None);
/// ```
///
/// To get quote in Russian with key `1000` :
///
/// ```
/// get_quote(Lang::RU, 1000);
/// ```
pub fn get_quote<T>(lang: Lang, key: T) -> Result<Quote>
where
    Option<u32>: From<T>,
{
    let url = match Option::from(key) {
        Some(k) => {
            if k.to_string().len() > 6 {
                return Err(Error::TooLongKey);
            }
            format!(
                "https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={}&key={}",
                lang,
                k
            )
        }
        None => format!(
            "https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={}",
            lang
        ),
    };
    let mut content = String::new();
    reqwest::get(&url)?.read_to_string(&mut content)?;
    let content = content.replace("\\'", "'");
    serde_json::from_str::<Quote>(content.as_str()).map_err(|e| -> Error {
        eprintln!("Parse Failed.");
        eprintln!("Please report to https://github.com/equal-l2/forismatic-rs with JSON!");
        eprintln!("JSON:\n{}", content);
        e.into()
    })
}
