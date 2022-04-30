use std::collections::HashMap;

pub static KEYWORD_PREFIX: char = '\u{29E}';

#[derive(Clone)]
pub enum Form<'a> {
    Nil,
    False,
    True,
    Int(i64),
    String(String),
    Keyword(String),
    Symbol(&'a str),
    List(Vec<Form<'a>>),
    Vector(Vec<Form<'a>>),
    Map(HashMap<String, Form<'a>>),
    Exec(&'a Exec),
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
    MissingMacroArgument,
    InvalidMetaMacro,
    InvalidType,
    MissingSymbol,
}

pub type Exec = for<'a> fn(Vec<Form<'a>>) -> Result<Form<'a>, FormError>;
pub type Env<'e> = HashMap<&'e str, Exec>;
