use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct Enum {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl Enum {
    pub(crate) fn new(item_id: usize, source_id: usize, identifier: Identifier, choices: Vec<EnumChoice>, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            identifier,
            choices,
            span,
            resolved: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EnumChoice {
    pub(crate) identifier: Identifier,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl EnumChoice {
    pub(crate) fn new(identifier: Identifier, span: Span) -> Self {
        Self { identifier, span, resolved: false }
    }
}
