use std::fmt::{Display, Formatter};
use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Pipeline {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("$")?;
        Display::fmt(&self.expression, f)
    }
}
