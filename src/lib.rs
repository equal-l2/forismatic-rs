extern crate reqwest;
#[macro_use]
extern crate serde_derive;

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
    TooLongKey,
}

impl std::convert::From<reqwest::Error> for self::Error {
    fn from(e: reqwest::Error) -> Self {
        Self{ kind : Kind::Reqwest(e) }
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
    let quote : Quote = reqwest::get(&url)?.json()?;
    Ok(quote)
}
