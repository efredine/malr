use crate::Form;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::iter::Peekable;
use std::vec::IntoIter;

// Todo: It would be better to return an iterator on the match.
// Returning the match with position rather than just the string because it will be useful
// for formatting errors.
pub fn tokenize(text: &str) -> Vec<Match> {
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

pub struct Reader<'a> {
    text: &'a str,
    // pub tokens: Vec<Match<'a>>,
    pub iter: Peekable<IntoIter<Match<'a>>>,
}

impl<'a> Reader<'a> {
    pub fn new(text: &'a str) -> Reader<'a> {
        let tokens = tokenize(text);
        let iter = tokens.into_iter().peekable();
        Reader { text, iter }
    }

    pub fn read_form(&mut self) -> Option<Result<Form<'a>, String>> {
        match self.iter.peek() {
            None => None,
            Some(token) => match token.as_str() {
                "(" => self.read_list(),
                ")" => return Some(Err(String::from("Missing opening bracket."))),
                _ => self.read_atom(),
            },
        }
    }

    fn read_list(&mut self) -> Option<Result<Form<'a>, String>> {
        self.iter.next();
        let mut list: Vec<Form> = Vec::new();
        while let Some(next_token) = self.iter.peek() {
            match next_token.as_str() {
                ")" => {
                    self.iter.next();
                    return Some(Ok(Form::List(list)));
                }
                _ => {
                    let next_form_option = self.read_form();
                    match next_form_option {
                        None => break,
                        Some(next_form_result) => match next_form_result {
                            Ok(form) => list.push(form),
                            Err(error) => return Some(Err(error)),
                        },
                    }
                }
            }
        }
        Some(Err(String::from("Missing closing bracket")))
    }

    fn read_atom(&mut self) -> Option<Result<Form<'a>, String>> {
        match self.iter.next() {
            None => None,
            Some(token) => Some(Ok(Form::Symbol(token.as_str()))),
        }
    }
}
