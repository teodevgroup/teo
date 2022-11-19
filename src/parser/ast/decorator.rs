use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;
use crate::parser::ast::unit::Unit;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub(crate) expression: ExpressionKind,
    pub(crate) span: Span,
}
