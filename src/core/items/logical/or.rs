use async_trait::async_trait;
use crate::core::result::Result;
use crate::core::item::Item;

use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct OrItem {
    value: Value
}

impl OrItem {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[async_trait]
impl Item for OrItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if ctx.value.is_null() {
            Ok(ctx.clone())
        } else {
            match &self.value {
                Value::Pipeline(p) => Ok(ctx.clone().with_value(p.process(ctx.clone()).await?)),
                _ => Ok(ctx.with_value(self.value.clone())),
            }
        }
    }
}
