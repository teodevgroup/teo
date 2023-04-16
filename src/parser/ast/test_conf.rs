use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct ASTTestConf {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) reset_after_find: Value,
}

impl ASTTestConf {
    pub(crate) fn new(items: Vec<Item>, span: Span, source_id: usize, item_id: usize) -> Self {
        Self {
            id: item_id, items, span, source_id, reset_after_find: Value::Null
        }
    }
}
