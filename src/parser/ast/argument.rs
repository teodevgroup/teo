use std::fmt::{Display, Formatter};
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct Argument {
    pub(crate) name: Option<ASTIdentifier>,
    pub(crate) value: ExpressionKind,
    pub(crate) span: Span,
    pub(crate) resolved: Option<Entity>,
}

impl Argument {
    pub(crate) fn get_value(&self) -> Option<&Value> {
        self.resolved.as_ref().map(|e| e.as_value()).flatten()
    }
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

#[derive(Debug, Clone, ToMut)]
pub struct ArgumentList {
    pub(crate) arguments: Vec<Argument>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl ArgumentList {
    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
}

impl Display for ArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        let len = self.arguments.len();
        for (index, expression) in self.arguments.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}
