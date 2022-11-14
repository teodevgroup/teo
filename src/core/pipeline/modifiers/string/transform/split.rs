use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct SplitModifier {
    separator: Value
}

impl SplitModifier {
    pub fn new(separator: impl Into<Value>) -> Self {
        Self { separator: separator.into() }
    }
}

#[async_trait]
impl Modifier for SplitModifier {

    fn name(&self) -> &'static str {
        "split"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.separator.resolve(ctx.clone()).await;
                let separator = arg.as_str().unwrap();
                ctx.alter_value(Value::Vec(s.split(separator).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>()))
            }
        }
    }
}
