use crate::parser::ast::accessible::Accessible;
use crate::parser::ast::reference::Reference;
use crate::prelude::Value;

pub(crate) enum Entity {
    Value(Value),
    Reference(Reference),
    Accessible(Accessible),
}

impl Entity {

    pub(crate) fn as_value(&self) -> Option<&Value> {
        match self {
            Entity::Value(v) => Some(v),
            _ => None,
        }
    }

    pub(crate) fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    pub(crate) fn as_reference(&self) -> Option<&Reference> {
        match self {
            Entity::Reference(v) => Some(v),
            _ => None,
        }
    }

    pub(crate) fn is_reference(&self) -> bool {
        self.as_reference().is_some()
    }

    pub(crate) fn as_accessible(&self) -> Option<&Accessible> {
        match self {
            Entity::Accessible(v) => Some(v),
            _ => None,
        }
    }

    pub(crate) fn is_accessible(&self) -> bool {
        self.as_accessible().is_some()
    }
}
