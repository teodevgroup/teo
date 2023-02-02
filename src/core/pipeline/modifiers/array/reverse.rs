use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct ReverseModifier {}

impl ReverseModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for ReverseModifier {

    fn name(&self) -> &'static str {
        "reverse"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match &ctx.value {
            Value::String(s) => ctx.alter_value(Value::String(s.chars().rev().collect::<String>())),
            Value::Vec(v) => ctx.alter_value(Value::Vec(v.into_iter().rev().map(|v| v.clone()).collect())),
            _ => ctx.invalid("Value cannot be reversed.")
        }
    }
}
