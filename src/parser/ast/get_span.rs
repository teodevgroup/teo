use super::span::Span;

pub trait GetSpan {
    /// The span of the node.
    fn span(&self) -> Span;
}
