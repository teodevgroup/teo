use std::fmt::{Display, Formatter};
use crate::parser::ast::argument::Argument;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::path::Path;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Call {
    pub(crate) path: Path,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) span: Span,
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.identifier, f)?;
        f.write_str("(")?;
        let len = self.arguments.len();
        for (index, argument) in self.arguments.iter().enumerate() {
            Display::fmt(argument, f)?;
            if index < len - 1 {
                f.write_str(", ")
            }
        }
        f.write_str(")")
    }
}
