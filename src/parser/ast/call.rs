use crate::parser::ast::argument::Argument;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Call {
    pub(crate) identifier: Identifier,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) span: Span,
}
