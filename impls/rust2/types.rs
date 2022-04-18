#[derive(Debug)]
pub enum Form<'a> {
    False,
    Int(i64),
    List(Vec<Form<'a>>),
    Vector(Vec<Form<'a>>),
    Nil,
    FormString(String),
    Symbol(&'a str),
    True,
}
