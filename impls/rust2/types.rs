use std::collections::HashMap;

pub static KEYWORD_PREFIX: char = '\u{29E}';

#[derive(Debug)]
pub enum Form<'a> {
    False,
    String(String),
    Int(i64),
    Keyword(String),
    List(Vec<Form<'a>>),
    Map(HashMap<String, Form<'a>>),
    Nil,
    Symbol(&'a str),
    True,
    Vector(Vec<Form<'a>>),
}

#[derive(Debug)]
pub enum FormError {
    MissingOpeningBracket,
    MissingTrailingBracket,
    MissingTrailingDoubleQuote,
    UnBalancedBackSlash,
    MissingKeywordValue,
    UnBalancedMap,
    InvalidKey,
}
