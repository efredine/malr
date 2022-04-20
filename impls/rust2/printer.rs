use crate::Form;

pub fn print(form: &Form) {
    println!("{}", format(form))
}

fn format(form: &Form) -> String {
    match form {
        Form::False => "false".to_string(),
        Form::String(source) => escape_str(source),
        Form::Int(i) => i.to_string(),
        Form::Keyword(str) => format!(":{}", str),
        Form::List(l) => {
            let string_list: Vec<String> = l.iter().map(format).collect();
            format!("({})", string_list.join(" "))
        }
        Form::Map(m) => {
            let pairs: Vec<String> = m
                .iter()
                .map(|(k, v)| format!("{} {}", escape_str(k), format(v)))
                .collect();
            format!("{{{}}}", pairs.join(" "))
        }
        Form::Nil => "Nil".to_string(),
        Form::Symbol(s) => s.to_string(),
        Form::True => "true".to_string(),
        Form::Vector(v) => {
            let vector_list: Vec<String> = v.iter().map(format).collect();
            format!("[{}]", vector_list.join(" "))
        }
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
