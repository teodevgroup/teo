use num_integer::Roots;

use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct SqrtModifier { }

impl SqrtModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for SqrtModifier {

    fn name(&self) -> &'static str {
        "sqrt"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value {
            Value::I8(v) => ctx.alter_value(Value::I8(v.sqrt())),
            Value::I16(v) => ctx.alter_value(Value::I16(v.sqrt())),
            Value::I32(v) => ctx.alter_value(Value::I32(v.sqrt())),
            Value::I64(v) => ctx.alter_value(Value::I64(v.sqrt())),
            Value::I128(v) => ctx.alter_value(Value::I128(v.sqrt())),
            Value::U8(v) => ctx.alter_value(Value::U8(v.sqrt())),
            Value::U16(v) => ctx.alter_value(Value::U16(v.sqrt())),
            Value::U32(v) => ctx.alter_value(Value::U32(v.sqrt())),
            Value::U64(v) => ctx.alter_value(Value::U64(v.sqrt())),
            Value::U128(v) => ctx.alter_value(Value::U128(v.sqrt())),
            Value::F32(v) => ctx.alter_value(Value::F32(v.sqrt())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.sqrt())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
