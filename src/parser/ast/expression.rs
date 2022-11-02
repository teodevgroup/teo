use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use crate::parser::ast::call::Call;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct NumericExpression {
    value: String,
    span: Span,
}

impl Display for NumericExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StringExpression {
    value: String,
    span: Span,
}

impl Display for StringExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BoolExpression {
    value: String,
    span: Span,
}

impl Display for BoolExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NullExpression {
    value: String,
    span: Span,
}

impl Display for NullExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnumChoiceExpression {
    value: String,
    span: Span,
}

impl Display for EnumChoiceExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")?;
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayExpression {
    expressions: Vec<Expression>,
    span: Span,
}

impl Display for ArrayExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")
    }
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

impl Display for DictionaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        let len = self.expressions.len();
        for (index, (key, expression)) in self.expressions.iter().enumerate() {
            f.write_str(key)?;
            f.write_str(": ")?;
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("}")
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Numeric(e) => Display::fmt(e, f),
            Expression::String(s) => Display::fmt(s, f),
            Expression::Bool(b) => Display::fmt(b, f),
            Expression::Null(n) => Display::fmt(n, f),
            Expression::EnumChoice(e) => Display::fmt(e, f),
            Expression::Identifier(i) => Display::fmt(i, f),
            Expression::Call(c) => Display::fmt(c, f),
            Expression::Array(a) => Display::fmt(a, f),
            Expression::Dictionary(d) => Display::fmt(d, f),
        }
    }
}
