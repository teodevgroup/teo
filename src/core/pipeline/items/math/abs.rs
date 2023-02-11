use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct AbsModifier {}

impl AbsModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for AbsModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value {
            Value::I32(v) => ctx.with_value(Value::I32(v.abs())),
            Value::I64(v) => ctx.with_value(Value::I64(v.abs())),
            Value::F32(v) => ctx.with_value(Value::F32(v.abs())),
            Value::F64(v) => ctx.with_value(Value::F64(v.abs())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
