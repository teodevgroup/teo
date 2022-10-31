use crate::parser::ast::call::Call;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub(crate) call: Call,
    pub(crate) span: Span,
}
