use crate::Form;
use std::ops::Add;

pub fn print(form: &Form) {
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
        Form::Vector(v) => {
            let vector_list: Vec<String> = v.iter().map(format).collect();
            format!("[{}]", vector_list.join(" "))
        }
        Form::False => "false".to_string(),
        Form::Nil => "Nil".to_string(),
        Form::FormString(source) => escape_str(source),
        Form::True => "true".to_string(),
    }
}

fn escape_str(source: &String) -> String {
    let mut result = String::from('"');
    for c in source.chars() {
        if c == '\n' {
            result.push('\\');
            result.push('n');
            continue;
        }
        if c == '"' || c == '\\' {
            result.push('\\');
        }
        result.push(c);
    }
    result.push('"');
    result
}
