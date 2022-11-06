use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use crate::parser::ast::call::Call;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::path::Path;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct NumericExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NumericExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StringExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for StringExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BoolExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for BoolExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NullExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NullExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnumChoiceExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for EnumChoiceExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")?;
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TupleExpression {
    pub(crate) expressions: Vec<Expression>,
    pub(crate) span: Span,
}

impl Display for TupleExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayExpression {
    pub(crate) expressions: Vec<Expression>,
    pub(crate) span: Span,
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
    pub(crate) expressions: HashMap<String, Expression>,
    pub(crate) span: Span,
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

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Numeric(NumericExpression),
    String(StringExpression),
    Bool(BoolExpression),
    Null(NullExpression),
    EnumChoice(EnumChoiceExpression),
    Path(Path),
    Call(Call),
    Tuple(TupleExpression),
    Array(ArrayExpression),
    Dictionary(DictionaryExpression),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Numeric(e) => Display::fmt(e, f),
            Expression::String(s) => Display::fmt(s, f),
            Expression::Bool(b) => Display::fmt(b, f),
            Expression::Null(n) => Display::fmt(n, f),
            Expression::EnumChoice(e) => Display::fmt(e, f),
            Expression::Path(p) => Display::fmt(p, f),
            Expression::Call(c) => Display::fmt(c, f),
            Expression::Tuple(t) => Display::fmt(t, f),
            Expression::Array(a) => Display::fmt(a, f),
            Expression::Dictionary(d) => Display::fmt(d, f),
        }
    }
}
