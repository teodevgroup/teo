use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct GetLengthModifier {}

impl GetLengthModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for GetLengthModifier {

    fn name(&self) -> &'static str {
        "getLength"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let len = match &ctx.value {
            Value::String(s) => s.len(),
            Value::Vec(v) => v.len(),
            _ => {
                return ctx.invalid("Value doesn't have length.");
            }
        };
        ctx.alter_value(Value::I64(len as i64))
    }
}
