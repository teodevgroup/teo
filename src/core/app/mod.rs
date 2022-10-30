use crate::parser::parser::parse;

pub struct App { }

impl App {
    pub fn load() -> () {
        parse("abc");
    }
}
