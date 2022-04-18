use regex::Match;

#[derive(Debug)]
pub enum Form<'a> {
    False,
    FormString(String),
    Int(i64),
    Keyword(String),
    List(Vec<Form<'a>>),
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
}
