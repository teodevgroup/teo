use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct TrimModifier {}

impl TrimModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for TrimModifier {

    fn name(&self) -> &'static str {
        "trim"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::String(ref s) => ctx.alter_value(Value::String(s.trim().to_owned())),
            _ => ctx.invalid("Value is not string.")
        }
    }
}
