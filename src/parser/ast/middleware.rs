use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct MiddlewareDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) span: Span,
}