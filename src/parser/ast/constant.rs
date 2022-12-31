use std::fmt::{Display, Formatter, Write};
use crate::parser::ast::expression::Expression;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::reference::Reference;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Constant {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) expression: Expression,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl Constant {
    pub(crate) fn new(item_id: usize, source_id: usize, identifier: Identifier, expression: Expression, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            identifier,
            expression,
            span,
            resolved: false,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("let ")?;
        Display::fmt(&self.identifier, f)?;
        f.write_str(" = ")?;
        Display::fmt(&self.expression, f)
    }
}
