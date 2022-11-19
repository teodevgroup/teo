use crate::parser::ast::client::Client;
use crate::parser::ast::config::Config;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;

#[derive(Debug)]
pub(crate) enum Top {
    Import(Import),
    Connector(Connector),
    Enum(Enum),
    Model(Model),
    Generator(Generator),
    Client(Client),
    Config(Config),
    Constant(Constant),
}

impl Top {
    pub(crate) fn id(&self) -> usize {
        match self {
            Top::Enum(e) => e.id,
            Top::Model(m) => m.id,
            _ => 0,
        }
    }

    pub(crate) fn as_import(&self) -> Option<&Import> {
        match self {
            Top::Import(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_connector(&self) -> Option<&Connector> {
        match self {
            Top::Connector(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_connector_mut(&mut self) -> Option<&mut Connector> {
        match self {
            Top::Connector(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_enum(&self) -> Option<&Enum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_model(&self) -> Option<&Model> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_generator(&self) -> Option<&Generator> {
        match self {
            Top::Generator(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_client(&self) -> Option<&Client> {
        match self {
            Top::Client(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_config(&self) -> Option<&Config> {
        match self {
            Top::Config(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_constant(&self) -> Option<&Constant> {
        match self {
            Top::Constant(c) => Some(c),
            _ => None,
        }
    }
}
