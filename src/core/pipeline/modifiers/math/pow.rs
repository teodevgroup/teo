use std::cmp::{min};
use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct PowModifier {
    argument: Argument
}

impl PowModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for PowModifier {

    fn name(&self) -> &'static str {
        "pow"
    }

    async fn call(&self, ctx: Context) -> Context {
        let argument = self.argument.resolve(ctx.clone()).await;
        let exp = argument.as_u32().unwrap();
        match ctx.value {
            Value::I8(v) => ctx.alter_value(Value::I8(v.pow(exp))),
            Value::I16(v) => ctx.alter_value(Value::I16(v.pow(exp))),
            Value::I32(v) => ctx.alter_value(Value::I32(v.pow(exp))),
            Value::I64(v) => ctx.alter_value(Value::I64(v.pow(exp))),
            Value::I128(v) => ctx.alter_value(Value::I128(v.pow(exp))),
            Value::U8(v) => ctx.alter_value(Value::U8(v.pow(exp))),
            Value::U16(v) => ctx.alter_value(Value::U16(v.pow(exp))),
            Value::U32(v) => ctx.alter_value(Value::U32(v.pow(exp))),
            Value::U64(v) => ctx.alter_value(Value::U64(v.pow(exp))),
            Value::U128(v) => ctx.alter_value(Value::U128(v.pow(exp))),
            Value::F32(v) => ctx.alter_value(Value::F32(v.powf(argument.as_f32().unwrap()))),
            Value::F64(v) => ctx.alter_value(Value::F64(v.powf(argument.as_f64().unwrap()))),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
