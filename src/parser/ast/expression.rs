use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use chrono::format::Numeric;
use crate::parser::ast::call::Call;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::path::Path;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub(crate) struct NumericExpression {
    pub(crate) value: String,
    pub(crate) span: Span,
    pub(crate) resolved: Option<Value>,
}

impl NumericExpression {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Self { value, span, resolved: None }
    }
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
    pub(crate) resolved: Option<Value>,
}

impl StringExpression {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Self { value, span, resolved: None }
    }
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
    pub(crate) resolved: Option<Value>,
}

impl BoolExpression {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Self { value, span, resolved: None }
    }
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
    pub(crate) resolved: Option<Value>,
}

impl NullExpression {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Self { value, span, resolved: None }
    }
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
    pub(crate) resolved: Option<Value>,
}

impl EnumChoiceExpression {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Self { value, span, resolved: None }
    }
}

impl Display for EnumChoiceExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")?;
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RangeExpression {
    pub(crate) closed: bool,
    pub(crate) expressions: Vec<Expression>,
    pub(crate) span: Span,
    pub(crate) resolved: Option<Vec<Value>>,
}

impl RangeExpression {
    pub(crate) fn new(closed: bool, expressions: Vec<Expression>, span: Span) -> Self {
        Self { closed, expressions, span, resolved: None }
    }
}

impl Display for RangeExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(if self.closed { "..." } else { ".." })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TupleExpression {
    pub(crate) expressions: Vec<Expression>,
    pub(crate) span: Span,
    pub(crate) resolved: Option<Vec<Expression>>,
}

impl TupleExpression {
    pub(crate) fn new(expressions: Vec<Expression>, span: Span) -> Self {
        Self { expressions, span, resolved: None }
    }
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
    pub(crate) resolved: Option<Vec<Expression>>,
}

impl ArrayExpression {
    pub(crate) fn new(expressions: Vec<Expression>, span: Span) -> Self {
        Self { expressions, span, resolved: None }
    }
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
    pub(crate) resolved: Option<HashMap<String, Expression>>,
}

impl DictionaryExpression {
    pub(crate) fn new(expressions: HashMap<String, Expression>, span: Span) -> Self {
        Self { expressions, span, resolved: None }
    }
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
    Range(RangeExpression),
    Tuple(TupleExpression),
    Array(ArrayExpression),
    Dictionary(DictionaryExpression),
    Path(Path),
    Call(Call),
    Pipeline(Pipeline),
}

impl Expression {
    pub(crate) fn as_numeric(&self) -> Option<&NumericExpression> {
        match self {
            Expression::Numeric(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_numeric_mut(&mut self) -> Option<&mut NumericExpression> {
        match self {
            Expression::Numeric(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_string(&self) -> Option<&StringExpression> {
        match self {
            Expression::String(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_string_mut(&mut self) -> Option<&mut StringExpression> {
        match self {
            Expression::String(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_bool(&self) -> Option<&BoolExpression> {
        match self {
            Expression::Bool(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_bool_mut(&mut self) -> Option<&mut BoolExpression> {
        match self {
            Expression::Bool(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_null(&self) -> Option<&NullExpression> {
        match self {
            Expression::Null(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_null_mut(&mut self) -> Option<&mut NullExpression> {
        match self {
            Expression::Null(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_enum_choice(&self) -> Option<&EnumChoiceExpression> {
        match self {
            Expression::EnumChoice(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_enum_choice_mut(&mut self) -> Option<&mut EnumChoiceExpression> {
        match self {
            Expression::EnumChoice(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_range(&self) -> Option<&RangeExpression> {
        match self {
            Expression::Range(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_range_mut(&mut self) -> Option<&mut RangeExpression> {
        match self {
            Expression::Range(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple(&self) -> Option<&TupleExpression> {
        match self {
            Expression::Tuple(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple_mut(&mut self) -> Option<&mut TupleExpression> {
        match self {
            Expression::Tuple(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array(&self) -> Option<&ArrayExpression> {
        match self {
            Expression::Array(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array_mut(&mut self) -> Option<&mut ArrayExpression> {
        match self {
            Expression::Array(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_dictionary(&self) -> Option<&DictionaryExpression> {
        match self {
            Expression::Dictionary(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_dictionary_mut(&mut self) -> Option<&mut DictionaryExpression> {
        match self {
            Expression::Dictionary(n) => Some(n),
            _ => None,
        }
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
            Expression::Path(p) => Display::fmt(p, f),
            Expression::Call(c) => Display::fmt(c, f),
            Expression::Pipeline(p) => Display::fmt(p, f),
            Expression::Range(r) => Display::fmt(r, f),
            Expression::Tuple(t) => Display::fmt(t, f),
            Expression::Array(a) => Display::fmt(a, f),
            Expression::Dictionary(d) => Display::fmt(d, f),
        }
    }
}
