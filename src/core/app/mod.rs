use crate::parser::parser::Parser;

pub struct App { }

impl App {
    pub fn load() -> () {
        let parser = Parser::new();
        parser.parse(Some("schema.teo"));
    }
}
