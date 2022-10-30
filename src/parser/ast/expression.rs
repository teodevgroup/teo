use std::collections::HashMap;
use crate::parser::ast::call::Call;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct NumericExpression {
    value: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct StringExpression {
    value: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct BoolExpression {
    value: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct NullExpression {
    value: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumChoiceExpression {
    value: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayExpression {
    expressions: Vec<Expression>,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct DictionaryExpression {
    expressions: HashMap<String, Expression>,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Numeric(NumericExpression),
    String(StringExpression),
    Bool(BoolExpression),
    Null(NullExpression),
    EnumChoice(EnumChoiceExpression),
    Identifier(Identifier),
    Call(Call),
    Array(ArrayExpression),
    Dictionary(DictionaryExpression),
}
