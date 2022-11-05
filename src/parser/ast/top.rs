use crate::parser::ast::client::Client;
use crate::parser::ast::config::Config;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;

#[derive(Debug)]
pub(crate) enum Top {
    Connector(Connector),
    Enum(Enum),
    Model(Model),
    Generator(Generator),
    Client(Client),
    Config(Config),
}

impl Top {
    pub(crate) fn id(&self) -> usize {
        match self {
            Top::Enum(e) => e.id,
            Top::Model(m) => m.id,
            _ => 0,
        }
    }
}
