use std::fmt::{Display, Formatter};
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ASTIdentifierPath {
    pub(crate) identifiers: Vec<ASTIdentifier>,
    pub(crate) span: Span,
}

impl ASTIdentifierPath {
    pub(crate) fn path(&self) -> Vec<String> {
        self.identifiers.iter().map(|i| i.name.clone()).collect()
    }
}
//
// impl Display for ASTIdentifierPath {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for identifier in self.identifiers.iter() {
//
//         }
//     }
// }
