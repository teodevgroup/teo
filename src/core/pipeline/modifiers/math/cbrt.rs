use num_integer::Roots;
use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct CbrtModifier { }

impl CbrtModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for CbrtModifier {

    fn name(&self) -> &'static str {
        "cbrt"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::I32(v) => ctx.alter_value(Value::I32(v.cbrt())),
            Value::I64(v) => ctx.alter_value(Value::I64(v.cbrt())),
            Value::F32(v) => ctx.alter_value(Value::F32(v.cbrt())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.cbrt())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
