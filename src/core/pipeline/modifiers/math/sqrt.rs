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

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::I32(v) => ctx.alter_value(Value::I32(v.sqrt())),
            Value::I64(v) => ctx.alter_value(Value::I64(v.sqrt())),
            Value::F32(v) => ctx.alter_value(Value::F32(v.sqrt())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.sqrt())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
