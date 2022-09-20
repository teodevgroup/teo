use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct TruncateModifier {
    argument: Argument,
}

impl TruncateModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self {
            argument: argument.into(),
        }
    }
}

#[async_trait]
impl Modifier for TruncateModifier {

    fn name(&self) -> &'static str {
        "truncate"
    }

    async fn call(&self, ctx: Context) -> Context {
        let argument = self.argument.resolve(ctx.clone()).await.as_usize().unwrap();
        match &ctx.value {
            Value::String(s) => ctx.alter_value(Value::String(s.chars().take(argument).collect())),
            Value::Vec(v) => ctx.alter_value(Value::Vec(v.iter().take(argument).map(|v| v.clone()).collect())),
            _ => ctx.invalid("Value is not string or vector.")
        }
    }
}
