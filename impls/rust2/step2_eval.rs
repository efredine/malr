mod printer;
mod reader;
mod types;

use crate::printer::print;
use crate::reader::Reader;
use crate::types::{Env, Exec, Form, MalError};
use std::collections::HashMap;
use std::io::{BufRead, Lines, StdinLock, Write};
use std::rc::Rc;
use std::{i64, io};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut env: Env = HashMap::new();
    env.insert("+", Form::Exec(Rc::from(int_add as Exec)));
    env.insert("-", Form::Exec(Rc::from(int_minus as Exec)));
    env.insert("*", Form::Exec(Rc::from(int_mul as Exec)));
    env.insert("/", Form::Exec(Rc::from(int_div as Exec)));

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

fn eval<'e>(form: Form, env: &'e Env<'e>) -> Result<Form, MalError> {
    match form {
        Form::List(l) => {
            if let Form::List(evaluated) = eval_ast(Form::List(l), env)? {
                if evaluated.len() > 1 {
                    match evaluated.get(0).unwrap() {
                        Form::Exec(exec) => exec(&evaluated[1..]),
                        _ => Ok(Form::List(evaluated)),
                    }
                } else {
                    Ok(Form::List(evaluated))
                }
            } else {
                Err(MalError::EvalListAstError)
            }
        }
        _ => eval_ast(form, env),
    }
}

fn eval_ast<'e>(form: Form, env: &'e Env<'e>) -> Result<Form, MalError> {
    match form {
        Form::Symbol(symbol) => match env.get(&*symbol) {
            None => Err(MalError::MissingSymbol),
            Some(form) => Ok(form.clone()),
        },
        Form::List(l) => Ok(Form::List(
            l.into_iter()
                .map(|f| eval(f.clone(), env))
                .collect::<Result<_, _>>()?,
        )),
        Form::Vector(l) => Ok(Form::Vector(
            l.into_iter()
                .map(|f| eval(f.clone(), env))
                .collect::<Result<_, _>>()?,
        )),
        Form::Map(m) => {
            let evaluated: HashMap<String, Form> = m
                .iter()
                .map(|(k, v)| match eval(v.clone(), env) {
                    Ok(evaluated) => Ok((k.to_string(), evaluated)),
                    Err(e) => Err(e),
                })
                .collect::<Result<HashMap<_, _>, _>>()?;
            Ok(Form::Map(Rc::from(evaluated)))
        }
        _ => Ok(form),
    }
}

fn int_add(v: &[Form]) -> Result<Form, MalError> {
    int_operation(v, |a, b| a + b)
}

fn int_minus(v: &[Form]) -> Result<Form, MalError> {
    int_operation(v, |a, b| a - b)
}

fn int_mul(v: &[Form]) -> Result<Form, MalError> {
    int_operation(v, |a, b| a * b)
}

fn int_div(v: &[Form]) -> Result<Form, MalError> {
    int_operation(v, |a, b| a / b)
}

fn int_operation<F>(v: &[Form], f: F) -> Result<Form, MalError>
where
    F: Fn(i64, i64) -> i64,
{
    let as_ints: Vec<i64> = v
        .iter()
        .map(|f| match f {
            Form::Int(i) => Ok(*i),
            _ => Err(MalError::InvalidType),
        })
        .collect::<Result<Vec<i64>, _>>()?;
    let result: Option<i64> = as_ints.into_iter().reduce(f);
    result.map(|i| Form::Int(i)).ok_or(MalError::InvalidType)
}
