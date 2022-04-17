mod reader;

use crate::reader::tokenize;
use std::io;
use std::io::{BufRead, Lines, StdinLock, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = read(&mut lines) {
        for token in tokenize(&line?) {
            print(eval(token.as_str()));
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
