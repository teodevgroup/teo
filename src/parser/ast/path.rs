use std::fmt::{Display, Formatter};
use crate::parser::ast::identifier::Identifier;
use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Path {
    pub(crate) identifiers: Vec<Identifier>,
    pub(crate) span: Span,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.identifiers.len();
        for (index, identifier) in self.identifiers.iter().enumerate() {
            Display::fmt(identifier, f)?;
            if index != len - 1 {
                f.write_str(".")?;
            }
        }
        Ok(())
    }
}
