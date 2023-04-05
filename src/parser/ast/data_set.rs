use crate::parser::ast::expression::DictionaryLiteral;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;

#[derive(Debug, Clone)]
pub struct DataSet {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) auto_seed: bool,
    pub(crate) groups: Vec<DataSetGroup>,

}

impl DataSet {
    pub(crate) fn new(items: Vec<Item>, span: Span, source_id: usize, item_id: usize) -> Self {
        Self {
            id: item_id, items, span, source_id, auto_seed: false, groups: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataSetGroup {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) records: Vec<DataSetRecord>,
}

impl DataSetGroup {
    pub(crate) fn new(source_id: usize, item_id: usize, identifier: Identifier, span: Span, items: Vec<Item>) -> Self {
        Self {
            id: item_id, items, span, source_id, records: vec![], identifier
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataSetRecord {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) span: Span,
    pub(crate) dictionary: DictionaryLiteral,
}

impl DataSetRecord {
    pub(crate) fn new(source_id: usize, item_id: usize, identifier: Identifier, span: Span, dictionary: DictionaryLiteral) -> Self {
        Self {
            id: item_id, source_id, identifier, span, dictionary
        }
    }
}