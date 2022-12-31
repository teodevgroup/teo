use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::parser::ast::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Generator {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
}
