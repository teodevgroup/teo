use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct CeilModifier {}

impl CeilModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for CeilModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value {
            Value::F32(v) => ctx.with_value(Value::F32(v.ceil())),
            Value::F64(v) => ctx.with_value(Value::F64(v.ceil())),
            _ => ctx.invalid("Value is not floating point number."),
        }
    }
}
