use std::fmt::{Display, Formatter};
use crate::parser::ast::argument::Argument;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Call {
    pub(crate) identifier: Identifier,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) span: Span,
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.identifier, f)?;
        f.write_str("(")?;
        for argument in self.arguments.iter() {
            Display::fmt(argument, f)?;
        }
        f.write_str(")")
    }
}
