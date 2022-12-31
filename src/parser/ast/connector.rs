use crate::core::database::name::DatabaseName;
use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::parser::ast::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Connector {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) provider: Option<DatabaseName>,
    pub(crate) url: Option<String>,
}

impl Connector {
    pub(crate) fn new(items: Vec<Item>, span: Span, source_id: usize, item_id: usize) -> Self {
        Self {
            id: item_id, items, span, source_id, provider: None, url: None,
        }
    }
}
