use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;

#[derive(Debug, Clone)]
pub struct DebugConf {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) log_queries: bool,
    pub(crate) log_seed_records: bool,
}

impl DebugConf {
    pub(crate) fn new(items: Vec<Item>, span: Span, source_id: usize, item_id: usize) -> Self {
        Self {
            id: item_id, items, span, source_id, log_queries: false, log_seed_records: false,
        }
    }
}
