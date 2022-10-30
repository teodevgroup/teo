use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::field::Field;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Model {
    pub(crate) identifier: Identifier,
    pub(crate) fields: Vec<Field>,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
}
