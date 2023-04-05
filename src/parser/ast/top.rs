use crate::parser::ast::client::Client;
use crate::parser::ast::config::ServerConfig;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::data_set::DataSet;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;

#[derive(Debug)]
pub(crate) enum Top {
    Import(Import),
    Constant(Constant),
    Enum(Enum),
    Model(Model),
    Connector(Connector),
    Generator(Generator),
    Client(Client),
    ServerConfig(ServerConfig),
    DataSet(DataSet),
}

impl Top {

    pub(crate) fn id(&self) -> usize {
        match self {
            Top::Import(i) => i.id,
            Top::Constant(c) => c.id,
            Top::Enum(e) => e.id,
            Top::Model(m) => m.id,
            Top::Connector(c) => c.id,
            Top::Generator(g) => g.id,
            Top::Client(c) => c.id,
            Top::ServerConfig(c) => c.id,
            Top::DataSet(d) => d.id,
        }
    }

    pub(crate) fn as_import(&self) -> Option<&Import> {
        match self {
            Top::Import(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_import_mut(&mut self) -> Option<&mut Import> {
        match self {
            Top::Import(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_import(&self) -> bool {
        self.as_import().is_some()
    }

    pub(crate) fn as_constant(&self) -> Option<&Constant> {
        match self {
            Top::Constant(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn as_constant_mut(&mut self) -> Option<&mut Constant> {
        match self {
            Top::Constant(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn is_constant(&self) -> bool {
        self.as_constant().is_some()
    }

    pub(crate) fn as_enum(&self) -> Option<&Enum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_enum_mut(&mut self) -> Option<&mut Enum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_enum(&self) -> bool {
        self.as_enum().is_some()
    }

    pub(crate) fn as_model(&self) -> Option<&Model> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_model_mut(&mut self) -> Option<&mut Model> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        self.as_model().is_some()
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

    pub(crate) fn is_connector(&self) -> bool {
        self.as_connector().is_some()
    }

    pub(crate) fn as_generator(&self) -> Option<&Generator> {
        match self {
            Top::Generator(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_generator_mut(&mut self) -> Option<&mut Generator> {
        match self {
            Top::Generator(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_generator(&self) -> bool {
        self.as_generator().is_some()
    }

    pub(crate) fn as_client(&self) -> Option<&Client> {
        match self {
            Top::Client(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_client_mut(&mut self) -> Option<&mut Client> {
        match self {
            Top::Client(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_client(&self) -> bool {
        self.as_client().is_some()
    }

    pub(crate) fn as_server_config(&self) -> Option<&ServerConfig> {
        match self {
            Top::ServerConfig(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_server_config_mut(&mut self) -> Option<&mut ServerConfig> {
        match self {
            Top::ServerConfig(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_server_config(&self) -> bool {
        self.as_server_config().is_some()
    }

    pub(crate) fn as_data_set(&self) -> Option<&DataSet> {
        match self {
            Top::DataSet(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_data_set_mut(&mut self) -> Option<&mut DataSet> {
        match self {
            Top::DataSet(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_data_set(&self) -> bool {
        self.as_data_set().is_some()
    }
}
