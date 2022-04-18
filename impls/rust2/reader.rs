use crate::types::FormError;
use crate::Form;
use crate::Form::FormString;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Reader<'a> {
    text: &'a str,
    pub iter: Peekable<IntoIter<Match<'a>>>,
}

static VEC_RIGHT: &str = "]";
static LIST_RIGHT: &str = ")";

impl<'a> Reader<'a> {
    pub fn new(text: &'a str) -> Reader<'a> {
        let tokens = tokenize(text);
        let iter = tokens.into_iter().peekable();
        Reader { text, iter }
    }

    pub fn read_form(&mut self) -> Option<Result<Form<'a>, FormError>> {
        match self.iter.peek() {
            None => None,
            Some(token) => match token.as_str() {
                "(" => self.read_list(LIST_RIGHT),
                "[" => self.read_list(VEC_RIGHT),
                ")" | "]" => Some(Err(FormError::MissingOpeningBracket)),
                _ => self.read_atom(),
            },
        }
    }

    fn read_list(&mut self, expected_close: &str) -> Option<Result<Form<'a>, FormError>> {
        // consume opening character
        self.iter.next();
        let mut list: Vec<Form> = Vec::new();
        while let Some(next_token) = self.iter.peek() {
            match next_token.as_str() {
                close if close == "]" || close == ")" => {
                    // consume closing bracket
                    self.iter.next();
                    return if close == expected_close {
                        if expected_close == LIST_RIGHT {
                            Some(Ok(Form::List(list)))
                        } else {
                            Some(Ok(Form::Vector(list)))
                        }
                    } else {
                        Some(Err(FormError::MissingTrailingBracket))
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
        Some(Err(FormError::MissingTrailingBracket))
    }

    fn read_atom(&mut self) -> Option<Result<Form<'a>, FormError>> {
        self.iter.next().map(|token| match token.as_str() {
            "false" => Ok(Form::False),
            "true" => Ok(Form::True),
            "Nil" => Ok(Form::Nil),
            keyword if keyword.starts_with(":") => {
                if keyword.len() == 1 {
                    Err(FormError::MissingKeywordValue)
                } else {
                    let mut str = keyword.to_string();
                    str.remove(0);
                    Ok(Form::Keyword(str))
                }
            }
            a_str if a_str.starts_with(r#"""#) => parse_string(a_str),
            numeric if numeric.parse::<i64>().is_ok() => {
                Ok(Form::Int(numeric.parse::<i64>().unwrap()))
            }
            symbol => Ok(Form::Symbol(symbol)),
        })
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

fn parse_string(a_str: &str) -> Result<Form, FormError> {
    let mut result = String::new();
    let mut iter = a_str.chars().peekable();

    // skip leading character double quote
    iter.next();
    while let Some(cur) = iter.next() {
        if iter.peek().is_none() {
            if cur == '"' {
                return Ok(Form::FormString(result));
            } else {
                break;
            }
        }
        if cur == '\\' {
            if iter.peek().is_none() {
                return Err(FormError::UnBalancedBackSlash);
            } else {
                let next = iter.next().unwrap();
                match next {
                    '\\' | '#' => result.push(next),
                    'n' => result.push('\n'),
                    _ => return Err(FormError::UnBalancedBackSlash),
                }
            }
        } else {
            result.push(cur);
        }
    }
    Err(FormError::MissingTrailingDoubleQuote)
}
