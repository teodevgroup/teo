use crate::parser::ast::expression::DictionaryLiteral;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::identifier_path::ASTIdentifierPath;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct ASTDataSet {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) ns_path: Vec<String>,
    pub(crate) span: Span,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) auto_seed: bool,
    pub(crate) notrack: bool,
    pub(crate) groups: Vec<ASTDataSetGroup>,

}

impl ASTDataSet {
    pub(crate) fn new(span: Span, source_id: usize, item_id: usize, identifier: ASTIdentifier, auto_seed: bool, notrack: bool, groups: Vec<ASTDataSetGroup>, ns_path: Vec<String>) -> Self {
        Self {
            id: item_id, span, source_id, auto_seed, groups, identifier, notrack, ns_path,
        }
    }

    pub(crate) fn path(&self) -> Vec<String> {
        let mut result = self.ns_path.clone();
        result.push(self.identifier.name.clone());
        result
    }
}

#[derive(Debug, Clone)]
pub struct ASTDataSetGroup {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifiers: ASTIdentifierPath,
    pub(crate) span: Span,
    pub(crate) records: Vec<ASTDataSetRecord>,
    pub(crate) model_id_path: Vec<usize>,
    pub(crate) resolved: bool,
}

impl ASTDataSetGroup {
    pub(crate) fn new(source_id: usize, item_id: usize, identifiers: ASTIdentifierPath, span: Span, records: Vec<ASTDataSetRecord>) -> Self {
        Self {
            id: item_id, span, source_id, identifiers, records, model_id_path: vec![], resolved: false,
        }
    }

    pub(crate) fn resolve(&mut self, model_id_path: Vec<usize>) {
        self.model_id_path = model_id_path;
        self.resolved = true;
    }
}

#[derive(Debug, Clone)]
pub struct ASTDataSetRecord {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) span: Span,
    pub(crate) dictionary: DictionaryLiteral,
    pub(crate) resolved: Option<Value>,
}

impl ASTDataSetRecord {
    pub(crate) fn new(source_id: usize, item_id: usize, identifier: ASTIdentifier, span: Span, dictionary: DictionaryLiteral) -> Self {
        Self {
            id: item_id, source_id, identifier, span, dictionary, resolved: None
        }
    }
}