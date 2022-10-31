use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

pub(crate) struct Enum {
    pub(crate) identifier: Identifier,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) span: Span,
}

pub(crate) struct EnumChoice {
    pub(crate) identifier: Identifier,
    pub(crate) span: Span,
}
