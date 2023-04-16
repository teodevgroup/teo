use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct ValidateWithItem {
    pipeline: Pipeline
}

impl ValidateWithItem {
    pub fn new(pipeline: Pipeline) -> Self {
        return ValidateWithItem {
            pipeline
        };
    }
}

#[async_trait]
impl Item for ValidateWithItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let _ = self.pipeline.process(ctx.clone()).await?;
        Ok(ctx)
    }
}
