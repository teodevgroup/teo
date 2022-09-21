use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct SplitModifier {
    separator: Argument
}

impl SplitModifier {
    pub fn new(separator: impl Into<Argument>) -> Self {
        Self { separator: separator.into() }
    }
}

#[async_trait]
impl Modifier for SplitModifier {

    fn name(&self) -> &'static str {
        "split"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not vector."),
            Some(s) => {
                let arg = self.separator.resolve(ctx.clone()).await;
                let separator = arg.as_str().unwrap();
                ctx.alter_value(Value::Vec(s.split(separator).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>()))
            }
        }
    }
}
