mod printer;
mod reader;
mod types;

use crate::printer::print;
use crate::reader::Reader;
use crate::types::{Env, Form, FormError};
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Lines, StdinLock, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut env: Env = HashMap::new();
    env.insert("+", map_int_add);
    while let Some(line_result) = read(&mut lines) {
        let line = line_result?;
        repl(&line, &env);
    }
    Ok(())
}

fn repl<'a, 'e: 'a>(line: &'a str, env: &'e Env<'e>) {
    let mut reader = Reader::new(line);
    while let Some(form_result) = reader.read_form() {
        match form_result {
            Ok(form) => {
                let evaluated_result = eval(form, &env);
                match evaluated_result {
                    Ok(form) => print(&form),
                    Err(_) => println!("Evaluation Error"),
                }
            }
            Err(_) => {
                println!("'.*\n.*(EOF|end of input|unbalanced).*'");
            }
        }
    }
}

fn read(lines: &mut Lines<StdinLock>) -> Option<std::io::Result<String>> {
    print!("{}", "user> ");
    let _ = io::stdout().flush();
    lines.next()
}

fn eval<'a, 'e: 'a>(form: Form<'a>, env: &'e Env<'e>) -> Result<Form<'a>, FormError> {
    match form {
        Form::List(l) => {
            if l.len() == 0 {
                Ok(Form::List(l))
            } else {
                let evaluated_result: Result<Vec<Form>, FormError> =
                    l.into_iter().map(|f| eval_ast(f, env)).collect();
                let evaluated = evaluated_result?;
                if evaluated.len() > 1 {
                    match evaluated.get(0).unwrap() {
                        Form::Exec(exec) => exec(evaluated[1..].to_vec()),
                        _ => Ok(Form::List(evaluated)),
                    }
                } else {
                    Ok(Form::List(evaluated))
                }
            }
        }
        _ => eval_ast(form, env),
    }
}

fn eval_ast<'a, 'e: 'a>(form: Form<'a>, env: &'e Env<'e>) -> Result<Form<'a>, FormError> {
    match form {
        Form::Symbol(symbol) => match env.get(symbol) {
            None => Err(FormError::MissingSymbol),
            Some(exec) => Ok(Form::Exec(exec)),
        },
        _ => Ok(form),
    }
}

fn map_int_add<'a, 'r>(v: Vec<Form<'a>>) -> Result<Form<'r>, FormError> {
    let as_ints: Result<Vec<i64>, _> = v
        .iter()
        .map(|f| match f {
            Form::Int(i) => Ok(*i),
            _ => Err(FormError::InvalidType),
        })
        .collect();
    let result: Option<i64> = as_ints?.into_iter().reduce(|a, b| a + b);
    result.map(|i| Form::Int(i)).ok_or(FormError::InvalidType)
}
