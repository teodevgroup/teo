use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct IsOddModifier {}

impl IsOddModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsOddModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value {
            Value::I32(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::I64(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::F32(v) => if v % 2.0 == 1.0 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::F64(v) => if v % 2.0 == 1.0 { ctx } else { ctx.invalid("Value is not odd.") },
            _ => ctx.invalid("Value is not number.")
        }
    }
}
