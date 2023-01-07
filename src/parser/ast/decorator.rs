use crate::parser::ast::accessible::Accessible;
use crate::parser::ast::argument::{Argument, ArgumentList};
use crate::parser::ast::expression::ExpressionKind;
use crate::parser::ast::span::Span;
use crate::parser::ast::unit::Unit;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub(crate) expression: ExpressionKind,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) accessible: Option<Accessible>,
    pub(crate) arguments: Option<ArgumentList>,
}

impl Decorator {
    pub(crate) fn new(expression: ExpressionKind, span: Span) -> Self {
        Self { expression, span, resolved: false, accessible: None, arguments: None }
    }

    pub(crate) fn get_argument_list(&self) -> Vec<Argument> {
        match &self.arguments {
            Some(argument_list) => argument_list.arguments.clone(),
            None => vec![],
        }
    }
}
