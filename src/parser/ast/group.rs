use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Group {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}
