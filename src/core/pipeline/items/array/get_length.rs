use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct GetLengthModifier {}

impl GetLengthModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for GetLengthModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let len = match &ctx.value {
            Value::String(s) => s.len(),
            Value::Vec(v) => v.len(),
            _ => {
                return ctx.invalid("Value doesn't have length.");
            }
        };
        ctx.with_value(Value::I64(len as i64))
    }
}
