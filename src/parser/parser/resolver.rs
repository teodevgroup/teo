use crate::parser::parser::Parser;

pub(crate) struct Resolver { }

impl Resolver {
    pub(crate) fn resolve_parser(parser: &mut Parser) {
        match &parser.connector {
            None => panic!("Connector is not defined."),
            Some(c) => c.lock().unwrap().,
        }
    }
}
