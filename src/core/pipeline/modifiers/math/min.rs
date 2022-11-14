use std::cmp::{max};
use async_trait::async_trait;
use crate::core::tson::Value;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct MinModifier {
    argument: Value
}

impl MinModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for MinModifier {

    fn name(&self) -> &'static str {
        "min"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match ctx.value {
            Value::I8(v) => ctx.alter_value(Value::I8(max(v, argument.as_i8().unwrap()))),
            Value::I16(v) => ctx.alter_value(Value::I16(max(v, argument.as_i16().unwrap()))),
            Value::I32(v) => ctx.alter_value(Value::I32(max(v, argument.as_i32().unwrap()))),
            Value::I64(v) => ctx.alter_value(Value::I64(max(v, argument.as_i64().unwrap()))),
            Value::I128(v) => ctx.alter_value(Value::I128(max(v, argument.as_i128().unwrap()))),
            Value::U8(v) => ctx.alter_value(Value::U8(max(v, argument.as_u8().unwrap()))),
            Value::U16(v) => ctx.alter_value(Value::U16(max(v, argument.as_u16().unwrap()))),
            Value::U32(v) => ctx.alter_value(Value::U32(max(v, argument.as_u32().unwrap()))),
            Value::U64(v) => ctx.alter_value(Value::U64(max(v, argument.as_u64().unwrap()))),
            Value::U128(v) => ctx.alter_value(Value::U128(max(v, argument.as_u128().unwrap()))),
            Value::F32(v) => ctx.alter_value(Value::F32(if v >= argument.as_f32().unwrap() { v } else { argument.as_f32().unwrap() })),
            Value::F64(v) => ctx.alter_value(Value::F64(if v >= argument.as_f64().unwrap() { v } else { argument.as_f64().unwrap() })),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
