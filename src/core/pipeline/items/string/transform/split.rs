use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
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
impl Item for SplitModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => ctx.internal_server_error("Value is not string."),
            Some(s) => {
                let arg = self.separator.resolve(ctx.clone()).await;
                let separator = arg.as_str().unwrap();
                ctx.with_value(Value::Vec(s.split(separator).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>()))
            }
        }
    }
}
