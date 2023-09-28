use std::sync::Mutex;
use crate::parser::ast::expression::DictionaryLiteral;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::identifier_path::ASTIdentifierPath;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct DataSet {
    pub(crate) path: Vec<usize>,
    pub(crate) ns_path: Vec<String>,
    pub(crate) string_path: Vec<String>,
    pub(crate) span: Span,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) auto_seed: bool,
    pub(crate) notrack: bool,
    pub(crate) groups: Vec<ASTDataSetGroup>,
}

impl DataSet {

    pub(crate) fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct DataSetGroupResolved {
    model_path: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct DataSetGroup {
    pub(crate) path: Vec<usize>,
    pub(crate) identifiers: Iden,
    pub(crate) span: Span,
    pub(crate) records: Vec<DataSetRecord>,
    pub(crate) resolved: Mutex<Option<DataSetGroupResolved>>,
}

#[derive(Debug, Clone)]
pub struct DataSetRecord {
    pub(crate) path: Vec<usize>,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) span: Span,
    pub(crate) dictionary: DictionaryLiteral,
    pub(crate) resolved: Mutex<Option<Value>>,
}

impl DataSetRecord {

    pub(crate) fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }
}