use super::span::Span;
use super::get_span::GetSpan;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Identifier {
    pub(crate) name: String,
    pub(crate) span: Span,
}
