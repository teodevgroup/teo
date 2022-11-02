use std::fmt::{Display, Formatter};
use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Identifier {
    pub(crate) name: String,
    pub(crate) span: Span,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
