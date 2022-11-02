use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;

#[derive(Debug)]
pub(crate) enum Top {
    Enum(Enum),
    Model(Model),
    // Connector(Connector),
    // Client(Client),
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
