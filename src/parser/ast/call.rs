use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Call {
    pub(crate) identifier: Identifier,
    pub(crate) span: Span,
}
