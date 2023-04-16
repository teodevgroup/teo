use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct MapItem {
    pipeline: Pipeline
}

impl MapItem {
    pub fn new(pipeline: Pipeline) -> Self {
        return MapItem {
            pipeline
        };
    }
}

#[async_trait]
impl Item for MapItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let mut retval = Vec::new();
        for (i, val) in ctx.value.as_vec().unwrap().iter().enumerate() {
            let item_ctx = ctx.with_value(val.clone()).with_path(&ctx.path + i);
            retval.push(self.pipeline.process(item_ctx).await?);
        }
        Ok(ctx.with_value(Value::Vec(retval)))
    }
}
