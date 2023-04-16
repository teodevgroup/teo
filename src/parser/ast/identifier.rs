use std::fmt::{Display, Formatter};
use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ASTIdentifier {
    pub(crate) name: String,
    pub(crate) span: Span,
}

impl Display for ASTIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
