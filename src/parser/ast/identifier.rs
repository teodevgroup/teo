use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Identifier {
    pub(crate) name: String,
    pub(crate) span: Span,
}
