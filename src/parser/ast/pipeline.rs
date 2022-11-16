use std::fmt::{Display, Formatter};
use crate::parser::ast::span::Span;
use crate::parser::ast::unit::Unit;

#[derive(Debug, Clone)]
pub(crate) struct Pipeline {
    pub(crate) unit: Unit,
    pub(crate) span: Span,
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("$")?;
        Display::fmt(&self.unit, f)
    }
}
