#[derive(Debug)]
pub enum Form<'a> {
    Int(isize),
    Symbol(&'a str),
    List(Vec<Form<'a>>),
}
