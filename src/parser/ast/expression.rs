use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use chrono::format::Numeric;

use crate::core::tson::range::Range;
use crate::parser::ast::call::Call;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::path::Path;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub(crate) struct NullishCoalescing {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for NullishCoalescing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(" ?? ")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NumericLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StringLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RegExpLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for RegExpLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BoolLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for BoolLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NullLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NullLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnumChoiceLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for EnumChoiceLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")?;
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RangeLiteral {
    pub(crate) closed: bool,
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for RangeLiteral {
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
pub(crate) struct TupleLiteral {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for TupleLiteral {
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
pub(crate) struct ArrayLiteral {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for ArrayLiteral {
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
pub(crate) struct DictionaryLiteral {
    pub(crate) expressions: Vec<(ExpressionKind, ExpressionKind)>,
    pub(crate) span: Span,
}

impl Display for DictionaryLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        let len = self.expressions.len();
        for (index, (key, expression)) in self.expressions.iter().enumerate() {
            Display::fmt(key, f)?;
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
pub(crate) enum ExpressionKind {
    NullishCoalescing(NullishCoalescing),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    RegExpLiteral(RegExpLiteral),
    BoolLiteral(BoolLiteral),
    NullLiteral(NullLiteral),
    EnumChoiceLiteral(EnumChoiceLiteral),
    RangeLiteral(RangeLiteral),
    TupleLiteral(TupleLiteral),
    ArrayLiteral(ArrayLiteral),
    DictionaryLiteral(DictionaryLiteral),
    Path(Path),
    Call(Call),
    Pipeline(Pipeline),
}

impl ExpressionKind {

    pub(crate) fn as_numeric_literal(&self) -> Option<&NumericLiteral> {
        match self {
            ExpressionKind::NumericLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_numeric_mut(&mut self) -> Option<&mut NumericLiteral> {
        match self {
            ExpressionKind::NumericLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_string(&self) -> Option<&StringLiteral> {
        match self {
            ExpressionKind::StringLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_string_mut(&mut self) -> Option<&mut StringLiteral> {
        match self {
            ExpressionKind::StringLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_regexp(&self) -> Option<&RegExpLiteral> {
        match self {
            ExpressionKind::RegExpLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_regexp_mut(&mut self) -> Option<&mut RegExpLiteral> {
        match self {
            ExpressionKind::RegExpLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_bool(&self) -> Option<&BoolLiteral> {
        match self {
            ExpressionKind::BoolLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_bool_mut(&mut self) -> Option<&mut BoolLiteral> {
        match self {
            ExpressionKind::BoolLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_null(&self) -> Option<&NullLiteral> {
        match self {
            ExpressionKind::NullLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_null_mut(&mut self) -> Option<&mut NullLiteral> {
        match self {
            ExpressionKind::NullLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_enum_choice(&self) -> Option<&EnumChoiceLiteral> {
        match self {
            ExpressionKind::EnumChoiceLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_enum_choice_mut(&mut self) -> Option<&mut EnumChoiceLiteral> {
        match self {
            ExpressionKind::EnumChoiceLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_range(&self) -> Option<&RangeLiteral> {
        match self {
            ExpressionKind::RangeLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_range_mut(&mut self) -> Option<&mut RangeLiteral> {
        match self {
            ExpressionKind::RangeLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple(&self) -> Option<&TupleLiteral> {
        match self {
            ExpressionKind::TupleLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple_mut(&mut self) -> Option<&mut TupleLiteral> {
        match self {
            ExpressionKind::TupleLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array(&self) -> Option<&ArrayLiteral> {
        match self {
            ExpressionKind::ArrayLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array_mut(&mut self) -> Option<&mut ArrayLiteral> {
        match self {
            ExpressionKind::ArrayLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_dictionary(&self) -> Option<&DictionaryLiteral> {
        match self {
            ExpressionKind::DictionaryLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_dictionary_mut(&mut self) -> Option<&mut DictionaryLiteral> {
        match self {
            ExpressionKind::DictionaryLiteral(n) => Some(n),
            _ => None,
        }
    }
}

impl Display for ExpressionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionKind::NumericLiteral(e) => Display::fmt(e, f),
            ExpressionKind::StringLiteral(s) => Display::fmt(s, f),
            ExpressionKind::RegExpLiteral(r) => Display::fmt(r, f),
            ExpressionKind::BoolLiteral(b) => Display::fmt(b, f),
            ExpressionKind::NullLiteral(n) => Display::fmt(n, f),
            ExpressionKind::EnumChoiceLiteral(e) => Display::fmt(e, f),
            ExpressionKind::RangeLiteral(r) => Display::fmt(r, f),
            ExpressionKind::TupleLiteral(t) => Display::fmt(t, f),
            ExpressionKind::ArrayLiteral(a) => Display::fmt(a, f),
            ExpressionKind::DictionaryLiteral(d) => Display::fmt(d, f),
            ExpressionKind::Path(p) => Display::fmt(p, f),
            ExpressionKind::Call(c) => Display::fmt(c, f),
            ExpressionKind::Pipeline(p) => Display::fmt(p, f),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) kind: ExpressionKind,
    pub(crate) resolved: Option<Value>,
}

impl Expression {
    pub(crate) fn new(kind: ExpressionKind) -> Self {
        Self { kind, resolved: None }
    }
}
