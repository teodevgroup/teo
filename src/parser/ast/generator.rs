use crate::core::app::environment::Environment;
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
    pub(crate) provider: Option<Environment>,
    pub(crate) dest: Option<String>,
}

impl Generator {
    pub(crate) fn new(id: usize, source_id: usize, identifier: Identifier, items: Vec<Item>, span: Span) -> Self {
        Self {
            id, source_id, identifier, items, span, provider: None, dest: None,
        }
    }
}
