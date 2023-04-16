use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct SplitItem {
    separator: Value
}

impl SplitItem {
    pub fn new(separator: impl Into<Value>) -> Self {
        Self { separator: separator.into() }
    }
}

#[async_trait]
impl Item for SplitItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.internal_server_error("split: value is not string")),
            Some(s) => {
                let arg = self.separator.resolve(ctx.clone()).await?;
                let separator = arg.as_str().unwrap();
                Ok(ctx.with_value(Value::Vec(s.split(separator).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>())))
            }
        }
    }
}
