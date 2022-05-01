use std::collections::HashMap;
use std::rc::Rc;

pub static KEYWORD_PREFIX: char = '\u{29E}';

#[derive(Clone)]
pub enum Form {
    Nil,
    False,
    True,
    Int(i64),
    String(Rc<str>),
    Keyword(Rc<str>),
    Symbol(Rc<str>),
    List(Rc<[Form]>),
    Vector(Rc<[Form]>),
    Map(Rc<HashMap<String, Form>>),
    Exec(Rc<Exec>),
}

#[derive(Debug)]
pub enum MalError {
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

pub type Exec = fn(&[Form]) -> Result<Form, MalError>;
pub type Env<'e> = HashMap<&'e str, Form>;
