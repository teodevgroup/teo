use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Subscript {
    pub(crate) expression: ExpressionKind,
    pub(crate) span: Span,
}
