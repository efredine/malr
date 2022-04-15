use std::io;
use std::io::{BufRead, Lines, StdinLock, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = read(&mut lines) {
        print(eval(&line?))
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
