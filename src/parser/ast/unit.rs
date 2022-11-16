use std::fmt::{Display, Formatter};
use crate::parser::ast::expression::ExpressionKind;

#[derive(Debug, Clone)]
pub(crate) struct Unit {
    pub(crate) expressions: Vec<ExpressionKind>,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.head, f)?;
        for item in self.expressions.iter() {
            if item.as_identifier().is_some() {
                f.write_str(".")?;
            }
            Display::fmt(&item, f)?;
        }
        Ok(())
    }
}
