use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct RoundModifier {}

impl RoundModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for RoundModifier {

    fn name(&self) -> &'static str {
        "round"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value {
            Value::F32(v) => ctx.alter_value(Value::F32(v.round())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.round())),
            _ => ctx.invalid("Value is not floating point number."),
        }
    }
}
