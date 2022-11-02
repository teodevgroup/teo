use crate::parser::ast::identifier::Identifier;

#[derive(Debug)]
pub(crate) enum Arity {
    Scalar,
    Array,
    Dictionary,
}

#[derive(Debug)]
pub(crate) struct Type {
    pub(crate) identifier: Identifier,
    pub(crate) arity: Arity,
    pub(crate) required: bool,
}
