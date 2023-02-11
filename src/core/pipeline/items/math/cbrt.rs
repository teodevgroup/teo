use num_integer::Roots;
use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct CbrtModifier { }

impl CbrtModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for CbrtModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value {
            Value::I32(v) => ctx.with_value(Value::I32(v.cbrt())),
            Value::I64(v) => ctx.with_value(Value::I64(v.cbrt())),
            Value::F32(v) => ctx.with_value(Value::F32(v.cbrt())),
            Value::F64(v) => ctx.with_value(Value::F64(v.cbrt())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
