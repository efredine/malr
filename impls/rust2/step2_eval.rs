mod printer;
mod reader;
mod types;

use crate::printer::print;
use crate::reader::Reader;
use crate::types::{Env, Form, FormError};
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Lines, StdinLock, Write};
use std::rc::Rc;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut env: Env = HashMap::new();
    env.insert("+", int_add);
    env.insert("-", int_minus);
    env.insert("*", int_mul);
    env.insert("/", int_div);

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
            Ok(form) => match eval(form, &env) {
                Ok(form) => print(&form),
                Err(e) => println!("{:?}", e),
            },
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
    // println!("evaluating");
    // print(&form);
    match form {
        Form::List(l) => {
            if let Form::List(evaluated) = eval_ast(Form::List(l), env)? {
                if evaluated.len() > 1 {
                    match evaluated.get(0).unwrap() {
                        Form::Exec(exec) => exec(evaluated[1..].to_vec()),
                        _ => Ok(Form::List(evaluated)),
                    }
                } else {
                    Ok(Form::List(evaluated))
                }
            } else {
                Err(FormError::EvalListAstError)
            }
        }
        _ => eval_ast(form, env),
    }
}

fn eval_ast<'a, 'e: 'a>(form: Form<'a>, env: &'e Env<'e>) -> Result<Form<'a>, FormError> {
    // println!("eval AST");
    // print(&form);
    match form {
        Form::Symbol(symbol) => match env.get(&*symbol) {
            None => Err(FormError::MissingSymbol),
            Some(exec) => Ok(Form::Exec(exec)),
        },
        Form::List(l) => Ok(Form::List(
            l.into_iter()
                .map(|f| eval(f, env))
                .collect::<Result<_, _>>()?,
        )),
        Form::Vector(l) => Ok(Form::Vector(
            l.into_iter()
                .map(|f| eval(f, env))
                .collect::<Result<_, _>>()?,
        )),
        Form::Map(m) => Ok(Form::Map(
            m.into_iter()
                .map(|(k, v)| match eval(v, env) {
                    Ok(evaluated) => Ok((k, evaluated)),
                    Err(e) => Err(e),
                })
                .collect::<Result<_, _>>()?,
        )),
        _ => Ok(form),
    }
}

fn int_add(v: Vec<Form>) -> Result<Form, FormError> {
    int_operation(v, |a, b| a + b)
}

fn int_minus(v: Vec<Form>) -> Result<Form, FormError> {
    int_operation(v, |a, b| a - b)
}

fn int_mul(v: Vec<Form>) -> Result<Form, FormError> {
    int_operation(v, |a, b| a * b)
}

fn int_div(v: Vec<Form>) -> Result<Form, FormError> {
    int_operation(v, |a, b| a / b)
}

fn int_operation<F>(v: Vec<Form>, f: F) -> Result<Form, FormError>
where
    F: Fn(i64, i64) -> i64,
{
    let as_ints: Vec<i64> = v
        .into_iter()
        .map(|f| match f {
            Form::Int(i) => Ok(*i),
            _ => Err(FormError::InvalidType),
        })
        .collect::<Result<Vec<i64>, _>>()?;
    let result: Option<i64> = as_ints.into_iter().reduce(f);
    result
        .map(|i| Form::Int(Rc::from(i)))
        .ok_or(FormError::InvalidType)
}
