use crate::parser::parser::Parser;

pub struct App { }

impl App {
    pub fn load() -> () {
        Parser::parse_file("");
    }
}
