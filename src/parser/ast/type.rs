use crate::parser::ast::identifier::Identifier;

pub(crate) enum Arity {
    Scalar,
    Array,
    Dictionary,
}

#[derive(Debug, Clone)]
pub(crate) struct Type {
    pub(crate) identifier: Identifier,
    pub(crate) arity: Arity,
    pub(crate) required: bool,
}
