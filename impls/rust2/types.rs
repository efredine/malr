use std::collections::HashMap;
use std::rc::Rc;

pub static KEYWORD_PREFIX: char = '\u{29E}';

#[derive(Clone, Debug)]
pub enum Form {
    Nil,
    False,
    True,
    Int(Rc<i64>),
    String(Rc<str>),
    Keyword(Rc<str>),
    Symbol(Rc<str>),
    List(Rc<[Form]>),
    Vector(Rc<[Form]>),
    Map(Rc<HashMap<String, Form>>),
    Exec(Rc<Exec>),
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

pub type Exec = fn(Vec<Form>) -> Result<Form, FormError>;
pub type Env<'e> = HashMap<&'e str, Form>;
