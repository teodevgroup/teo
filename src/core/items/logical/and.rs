use async_trait::async_trait;

use crate::core::item::Item;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct AndItem {
    value: Value
}

impl AndItem {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[async_trait]
impl Item for AndItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        if !ctx.get_value().is_null() {
            Ok(ctx.clone())
        } else {
            match &self.value {
                Value::Pipeline(p) => Ok(ctx.clone().with_value(p.process(ctx.clone()).await?)),
                _ => Ok(ctx.with_value(self.value.clone())),
            }
        }
    }
}
