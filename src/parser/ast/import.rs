use std::collections::HashMap;
use std::path::PathBuf;
use maplit::hashmap;
use crate::parser::ast::expression::StringLiteral;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::reference::Reference;
use crate::parser::ast::span::Span;

#[derive(Clone, Debug)]
pub(crate) struct Import {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifiers: Vec<ASTIdentifier>,
    pub(crate) source: StringLiteral,
    pub(crate) path: PathBuf,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) from_id: Option<usize>,
    pub(crate) references: HashMap<String, Reference>,
}

impl Import {

    pub(crate) fn new(item_id: usize, source_id: usize, identifiers: Vec<ASTIdentifier>, source: StringLiteral, path: PathBuf, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            identifiers,
            source,
            path,
            span,
            resolved: false,
            from_id: None,
            references: hashmap!{},
        }
    }
}
