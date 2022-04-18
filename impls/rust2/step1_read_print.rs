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
                Ok(form) => println!("{:?}", form),
                Err(err) => {
                    println!("Error: {}", err);
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

fn eval(str: &str) -> &str {
    str
}

fn print(str: &str) {
    println!("{}", str)
}
