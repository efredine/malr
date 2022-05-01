use std::collections::HashMap;
use std::rc::Rc;

pub static KEYWORD_PREFIX: char = '\u{29E}';

#[derive(Clone)]
pub enum Form<'a> {
    Nil,
    False,
    True,
    Int(Rc<i64>),
    String(Rc<str>),
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
    EvalListAstError,
}

pub type Exec = for<'a> fn(Vec<Form<'a>>) -> Result<Form<'a>, FormError>;
pub type Env<'e> = HashMap<&'e str, Exec>;
