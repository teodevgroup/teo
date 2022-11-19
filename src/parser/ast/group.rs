use crate::parser::ast::expression::Expression;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Group {
    pub(crate) expression: Box<Expression>,
    pub(crate) span: Span,
}
