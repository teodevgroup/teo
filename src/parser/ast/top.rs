use crate::parser::ast::action::ActionGroupDeclaration;
use crate::parser::ast::client::ASTClient;
use crate::parser::ast::config::ASTServer;
use crate::parser::ast::connector::ASTConnector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::data_set::DataSet;
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::interface::InterfaceDeclaration;
use crate::parser::ast::middleware::MiddlewareDeclaration;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::r#enum::ASTEnum;
use crate::parser::ast::test_conf::ASTTestConf;

#[derive(Debug)]
pub(crate) enum Top {
    Import(ASTImport),
    Constant(Constant),
    Enum(ASTEnum),
    Model(ASTModel),
    Connector(ASTConnector),
    Generator(ASTEntity),
    Client(ASTClient),
    ServerConfig(ASTServer),
    DataSet(DataSet),
    TestConf(ASTTestConf),
    DebugConf(ASTDebugConf),
    MiddlewareDeclaration(MiddlewareDeclaration),
    ActionGroupDeclaration(ActionGroupDeclaration),
    InterfaceDeclaration(InterfaceDeclaration),
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
            Top::TestConf(t) => t.id,
            Top::DebugConf(d) => d.id,
            Top::MiddlewareDeclaration(m) => m.id,
            Top::ActionGroupDeclaration(a) => a.id,
            Top::InterfaceDeclaration(i) => i.id,
        }
    }

    pub(crate) fn as_import(&self) -> Option<&ASTImport> {
        match self {
            Top::Import(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_import_mut(&mut self) -> Option<&mut ASTImport> {
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

    pub(crate) fn as_enum(&self) -> Option<&ASTEnum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_enum_mut(&mut self) -> Option<&mut ASTEnum> {
        match self {
            Top::Enum(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_enum(&self) -> bool {
        self.as_enum().is_some()
    }

    pub(crate) fn as_model(&self) -> Option<&ASTModel> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_model_mut(&mut self) -> Option<&mut ASTModel> {
        match self {
            Top::Model(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        self.as_model().is_some()
    }

    pub(crate) fn as_connector(&self) -> Option<&ASTConnector> {
        match self {
            Top::Connector(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_connector_mut(&mut self) -> Option<&mut ASTConnector> {
        match self {
            Top::Connector(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_connector(&self) -> bool {
        self.as_connector().is_some()
    }

    pub(crate) fn as_generator(&self) -> Option<&ASTEntity> {
        match self {
            Top::Generator(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_generator_mut(&mut self) -> Option<&mut ASTEntity> {
        match self {
            Top::Generator(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_generator(&self) -> bool {
        self.as_generator().is_some()
    }

    pub(crate) fn as_client(&self) -> Option<&ASTClient> {
        match self {
            Top::Client(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_client_mut(&mut self) -> Option<&mut ASTClient> {
        match self {
            Top::Client(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn is_client(&self) -> bool {
        self.as_client().is_some()
    }

    pub(crate) fn as_server_config(&self) -> Option<&ASTServer> {
        match self {
            Top::ServerConfig(i) => Some(i),
            _ => None
        }
    }

    pub(crate) fn as_server_config_mut(&mut self) -> Option<&mut ASTServer> {
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

    pub(crate) fn as_test_conf(&self) -> Option<&ASTTestConf> {
        match self {
            Top::TestConf(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_test_conf_mut(&mut self) -> Option<&mut ASTTestConf> {
        match self {
            Top::TestConf(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_test_conf(&self) -> bool {
        self.as_test_conf().is_some()
    }

    pub(crate) fn as_debug_conf(&self) -> Option<&ASTDebugConf> {
        match self {
            Top::DebugConf(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_debug_conf_mut(&mut self) -> Option<&mut ASTDebugConf> {
        match self {
            Top::DebugConf(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_debug_conf(&self) -> bool {
        self.as_debug_conf().is_some()
    }

    pub(crate) fn as_middleware(&self) -> Option<&MiddlewareDeclaration> {
        match self {
            Top::MiddlewareDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn as_middleware_mut(&mut self) -> Option<&mut MiddlewareDeclaration> {
        match self {
            Top::MiddlewareDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_middleware(&self) -> bool {
        self.as_middleware().is_some()
    }

    pub(crate) fn as_action_group(&self) -> Option<&ActionGroupDeclaration> {
        match self {
            Top::ActionGroupDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn as_action_group_mut(&mut self) -> Option<&mut ActionGroupDeclaration> {
        match self {
            Top::ActionGroupDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_action_group(&self) -> bool {
        self.as_action_group().is_some()
    }

    pub(crate) fn as_interface(&self) -> Option<&InterfaceDeclaration> {
        match self {
            Top::InterfaceDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn as_interface_mut(&mut self) -> Option<&mut InterfaceDeclaration> {
        match self {
            Top::InterfaceDeclaration(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn is_interface(&self) -> bool {
        self.as_interface().is_some()
    }
}
