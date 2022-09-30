use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct CeilModifier {}

impl CeilModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for CeilModifier {

    fn name(&self) -> &'static str {
        "ceil"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::F32(v) => ctx.alter_value(Value::F32(v.ceil())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.ceil())),
            _ => ctx.invalid("Value is not floating point number."),
        }
    }
}
