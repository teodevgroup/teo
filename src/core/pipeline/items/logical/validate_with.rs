use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct ValidateWithModifier {
    pipeline: Pipeline
}

impl ValidateWithModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ValidateWithModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Item for ValidateWithModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let _ = self.pipeline.process(ctx.clone()).await?;
        Ok(ctx)
    }
}
