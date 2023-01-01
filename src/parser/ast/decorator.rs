use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;
use crate::parser::ast::unit::Unit;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub(crate) expression: ExpressionKind,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl Decorator {
    pub(crate) fn new(expression: ExpressionKind, span: Span) -> Self {
        Self { expression, span, resolved: false }
    }
}
