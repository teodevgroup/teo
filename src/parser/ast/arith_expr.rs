use std::fmt::{Display, Formatter};
use crate::parser::ast::expression::ExpressionKind;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    BitNeg,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Neg => f.write_str("-"),
            Op::Add => f.write_str("+"),
            Op::Sub => f.write_str("-"),
            Op::Mul => f.write_str("*"),
            Op::Div => f.write_str("/"),
            Op::Mod => f.write_str("%"),
            Op::BitAnd => f.write_str("&"),
            Op::BitXor => f.write_str("^"),
            Op::BitOr => f.write_str("|"),
            Op::BitNeg => f.write_str("~"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ArithExpr {
    Expression(Box<ExpressionKind>),
    UnaryNeg(Box<ExpressionKind>),
    UnaryBitNeg(Box<ExpressionKind>),
    BinaryOp {
        lhs: Box<ArithExpr>,
        op: Op,
        rhs: Box<ArithExpr>,
    },
}

impl Display for ArithExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithExpr::UnaryNeg(e) => Display::fmt(&e, f),
            ArithExpr::UnaryBitNeg(e) => Display::fmt(&e, f),
            ArithExpr::Expression(e) => Display::fmt(&e, f),
            ArithExpr::BinaryOp { lhs, op, rhs } => {
                Display::fmt(&lhs, f)?;
                f.write_str(" ")?;
                Display::fmt(op, f)?;
                f.write_str(" ")?;
                Display::fmt(&rhs, f)?;
                Ok(())
            }
        }
    }
}
