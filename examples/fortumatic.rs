fn main() {
    let quote = forismatic::get_quote(forismatic::Lang::EN, 1000).unwrap();
    println!("{}", quote.quote_text);
    println!("\t-- {}", quote.quote_author);
    println!("URL: {}", quote.quote_link);
}
