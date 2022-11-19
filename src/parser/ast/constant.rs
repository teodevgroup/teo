use std::fmt::{Display, Formatter, Write};
use crate::parser::ast::expression::Expression;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Constant {
    pub(crate) identifier: Identifier,
    pub(crate) expression: Expression,
    pub(crate) span: Span,
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("let ")?;
        Display::fmt(&self.identifier, f)?;
        f.write_str(" = ")?;
        Display::fmt(&self.expression, f)
    }
}
