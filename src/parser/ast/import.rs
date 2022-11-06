use crate::parser::ast::expression::StringExpression;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

pub(crate) struct Import {
    pub(crate) identifiers: Vec<Identifier>,
    pub(crate) source: StringExpression,
    pub(crate) span: Span,
}
