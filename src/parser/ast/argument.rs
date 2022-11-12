use std::fmt::{Display, Formatter};
use crate::core::pipeline::argument::FunctionArgument;
use crate::parser::ast::expression::Expression;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Argument {
    pub(crate) name: Option<Identifier>,
    pub(crate) value: Expression,
    pub(crate) span: Span,
    pub(crate) resolved: Option<FunctionArgument>,
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(&name.name)?;
            f.write_str(": ")?;
        }
        Display::fmt(&self.value, f)
    }
}
