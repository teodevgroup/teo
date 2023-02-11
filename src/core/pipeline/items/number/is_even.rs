use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct IsEvenModifier {}

impl IsEvenModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsEvenModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value {
            Value::I32(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::I64(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::F32(v) => if v % 2.0 == 0.0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::F64(v) => if v % 2.0 == 0.0 { ctx } else { ctx.invalid("Value is not even.") },
            _ => ctx.invalid("Value is not number.")
        }
    }
}
