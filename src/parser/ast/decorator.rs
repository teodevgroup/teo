use crate::parser::ast::span::Span;
use crate::parser::ast::unit::Unit;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub(crate) unit: Unit,
    pub(crate) span: Span,
}
