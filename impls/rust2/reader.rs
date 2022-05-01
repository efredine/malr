use crate::types::{MalError, KEYWORD_PREFIX};
use crate::Form;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::Rc;
use std::vec::IntoIter;

pub struct Reader<'a> {
    pub iter: Peekable<IntoIter<Match<'a>>>,
}

static VEC_RIGHT: &str = "]";
static LIST_RIGHT: &str = ")";
static MAP_RIGHT: &str = "}";

static QUOTE: &str = "quote";
static UNQUOTE: &str = "unquote";
static QUASI_QUOTE: &str = "quasiquote";
static SPLICE_UNQUOTE: &str = "splice-unquote";
static DEREF: &str = "deref";
static WITH_META: &str = "with-meta";

impl<'a> Reader<'a> {
    pub fn new(text: &'a str) -> Reader<'a> {
        let tokens = tokenize(text);
        let iter = tokens.into_iter().peekable();
        Reader { iter }
    }

    pub fn read_form(&mut self) -> Option<Result<Form, MalError>> {
        match self.iter.peek() {
            None => None,
            Some(token) => match token.as_str() {
                "(" => self.read_list(LIST_RIGHT),
                "[" => self.read_list(VEC_RIGHT),
                "{" => self.read_list(MAP_RIGHT),
                "'" => self.read_macro(QUOTE),
                "~" => self.read_macro(UNQUOTE),
                "`" => self.read_macro(QUASI_QUOTE),
                "~@" => self.read_macro(SPLICE_UNQUOTE),
                "@" => self.read_macro(DEREF),
                "^" => self.read_meta_macro(),
                ")" | "]" | "}" => Some(Err(MalError::MissingOpeningBracket)),
                _ => self.read_atom(),
            },
        }
    }

    fn read_list(&mut self, expected_close: &str) -> Option<Result<Form, MalError>> {
        // consume opening character
        self.iter.next();
        let mut list: Vec<Form> = Vec::new();
        while let Some(next_token) = self.iter.peek() {
            match next_token.as_str() {
                close if close == "]" || close == ")" || close == "}" => {
                    // consume closing bracket
                    self.iter.next();
                    return if close == expected_close {
                        if expected_close == VEC_RIGHT {
                            Some(Ok(Form::Vector(Rc::from(list))))
                        } else if expected_close == MAP_RIGHT {
                            map_from_vec(list)
                        } else {
                            Some(Ok(Form::List(Rc::from(list))))
                        }
                    } else {
                        Some(Err(MalError::MissingTrailingBracket))
                    };
                }
                _ => match self.read_form() {
                    None => break,
                    Some(next_form_result) => match next_form_result {
                        Ok(form) => list.push(form),
                        Err(error) => return Some(Err(error)),
                    },
                },
            }
        }
        Some(Err(MalError::MissingTrailingBracket))
    }

    fn read_atom(&mut self) -> Option<Result<Form, MalError>> {
        self.iter.next().map(|token| match token.as_str() {
            "false" => Ok(Form::False),
            "true" => Ok(Form::True),
            "Nil" => Ok(Form::Nil),
            keyword if keyword.starts_with(":") => {
                if keyword.len() == 1 {
                    Err(MalError::MissingKeywordValue)
                } else {
                    let mut str = keyword.to_string();
                    str.remove(0);
                    str.insert(0, KEYWORD_PREFIX);
                    Ok(Form::Keyword(Rc::from(str)))
                }
            }
            a_string if a_string.starts_with(r#"""#) => parse_string(a_string),
            numeric if numeric.parse::<i64>().is_ok() => {
                Ok(Form::Int(Rc::from(numeric.parse::<i64>().unwrap())))
            }
            symbol => Ok(Form::Symbol(Rc::from(symbol))),
        })
    }

    fn read_macro(&mut self, macro_fn: &'static str) -> Option<Result<Form, MalError>> {
        // consume macro token
        self.iter.next();
        return if let Some(argument_result) = self.read_form() {
            Some(Ok(Form::List(Rc::from(vec![
                Form::Symbol(Rc::from(macro_fn)),
                argument_result.ok()?,
            ]))))
        } else {
            Some(Err(MalError::MissingMacroArgument))
        };
    }

    fn read_meta_macro(&mut self) -> Option<Result<Form, MalError>> {
        // consume macro token
        self.iter.next();
        return if self.iter.peek().is_none() {
            Some(Err(MalError::MissingMacroArgument))
        } else {
            let mut list: Vec<Form> = Vec::new();
            while let Some(form_result) = self.read_form() {
                list.push(form_result.ok()?)
            }
            if list.len() != 2 {
                Some(Err(MalError::InvalidMetaMacro))
            } else {
                list.push(Form::Symbol(Rc::from(WITH_META)));
                list.reverse();
                Some(Ok(Form::List(Rc::from(list))))
            }
        };
    }
}

// Todo: It would be better to return an iterator on the match.
// Returning the match with position rather than just the string because it will be useful
// for formatting errors.
fn tokenize(text: &str) -> Vec<Match> {
    // Using lazy static for the regex accomplishes two things:
    // 1. The regex is only compiled once;
    // 2. There are no lifetime issues with returning an iterator on the captured results.
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
                .unwrap();
    }

    // There will only ever be one capture group for each match because that's how the regex
    // is constructed. There should be one match for each type of token. The call to capture.get
    // returns an Option<Match> and filter_map filters out the None values.
    RE.captures_iter(&text)
        .filter_map(|capture| capture.get(1))
        .collect()
}

fn parse_string(a_str: &str) -> Result<Form, MalError> {
    let mut result = String::new();
    let mut iter = a_str.chars().peekable();

    // consume leading character double quote
    iter.next();
    while let Some(cur) = iter.next() {
        if iter.peek().is_none() {
            if cur == '"' {
                return Ok(Form::String(Rc::from(result)));
            } else {
                break;
            }
        }
        if cur == '\\' {
            if iter.peek().is_none() {
                return Err(MalError::UnBalancedBackSlash);
            } else {
                let next = iter.next().unwrap();
                match next {
                    '\\' | '"' => result.push(next),
                    'n' => result.push('\n'),
                    _ => return Err(MalError::UnBalancedBackSlash),
                }
            }
        } else {
            result.push(cur);
        }
    }
    Err(MalError::MissingTrailingDoubleQuote)
}

fn map_from_vec(list: Vec<Form>) -> Option<Result<Form, MalError>> {
    let mut map: HashMap<String, Form> = HashMap::new();
    let mut iter = list.into_iter();
    while let Some(key) = iter.next() {
        if let Some(value) = iter.next() {
            match key {
                Form::String(s) => map.insert(s.to_string(), value),
                Form::Keyword(s) => map.insert(s.to_string(), value),
                _ => return Some(Err(MalError::InvalidKey)),
            };
        } else {
            return Some(Err(MalError::UnBalancedMap));
        };
    }
    Some(Ok(Form::Map(Rc::from(map))))
}
