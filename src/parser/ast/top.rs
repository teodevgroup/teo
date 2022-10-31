use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;

#[derive(Debug, Clone)]
pub enum Top {
    Enum(Enum),
    Model(Model),
    // Connector(Connector),
    // Client(Client),
}
