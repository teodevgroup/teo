use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::parser::ast::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Connector {
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
}
