mod reader;
mod types;

use crate::reader::Reader;
use crate::types::Form;
use std::io;
use std::io::{BufRead, Lines, StdinLock, Write};
use std::process::exit;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = read(&mut lines) {
        let line_result = line?;
        let mut reader = Reader::new(&line_result);
        while let Some(form_result) = reader.read_form() {
            match form_result {
                Ok(form) => print(eval(&form)),
                Err(err) => {
                    eprintln!("Error: {}", err);
                    exit(1);
                }
            }
        }
    }
    Ok(())
}

fn read(lines: &mut Lines<StdinLock>) -> Option<std::io::Result<String>> {
    print!("{}", "user> ");
    let _ = io::stdout().flush();
    lines.next()
}

fn eval<'a>(form: &'a Form) -> &'a Form<'a> {
    form
}

fn print(form: &Form) {
    println!("{}", format(form))
}

fn format(form: &Form) -> String {
    match form {
        Form::Int(i) => i.to_string(),
        Form::Symbol(s) => s.to_string(),
        Form::List(l) => {
            let string_list: Vec<String> = l.iter().map(format).collect();
            format!("({})", string_list.join(" "))
        }
    }
}
