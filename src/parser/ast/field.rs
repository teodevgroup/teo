use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::r#type::Type;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) identifier: Identifier,
    pub(crate) r#type: Type,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
}
