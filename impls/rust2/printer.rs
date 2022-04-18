use crate::Form;

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
        Form::False => "false".to_string(),
        Form::Nil => "Nil".to_string(),
        Form::FormString(a_string) => format!(r#""{}""#, a_string),
        Form::True => "true".to_string(),
    }
}
