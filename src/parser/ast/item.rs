use crate::parser::ast::expression::Expression;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub(crate) identifier: ASTIdentifier,
    pub(crate) expression: Expression,
    pub(crate) span: Span,
}
