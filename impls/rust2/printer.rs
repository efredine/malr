use crate::types::KEYWORD_PREFIX;
use crate::Form;

pub fn print(form: &Form) {
    println!("{}", format(form))
}

fn format(form: &Form) -> String {
    match form {
        Form::Nil => "Nil".to_string(),
        Form::True => "true".to_string(),
        Form::False => "false".to_string(),
        Form::Int(i) => i.to_string(),
        Form::Symbol(symbol) => symbol.to_string(),
        Form::String(source) => format_string(source),
        Form::Keyword(keyword) => format_string(keyword),
        Form::List(l) => {
            let string_list: Vec<String> = l.iter().map(format).collect();
            format!("({})", string_list.join(" "))
        }
        Form::Vector(v) => {
            let vector_list: Vec<String> = v.iter().map(format).collect();
            format!("[{}]", vector_list.join(" "))
        }
        Form::Map(m) => {
            let pairs: Vec<String> = m
                .iter()
                .map(|(k, v)| format!("{} {}", format_string(k), format(v)))
                .collect();
            format!("{{{}}}", pairs.join(" "))
        }
        Form::Exec(_) => "exec".to_string(),
    }
}

fn format_string(source: &str) -> String {
    return if source.starts_with(KEYWORD_PREFIX) {
        let mut result = source.to_string();
        result.remove(0);
        result.insert(0, ':');
        result
    } else {
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
    };
}
