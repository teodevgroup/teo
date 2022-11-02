use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct Enum {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) span: Span,
}

#[derive(Debug)]
pub(crate) struct EnumChoice {
    pub(crate) identifier: Identifier,
    pub(crate) span: Span,
}
