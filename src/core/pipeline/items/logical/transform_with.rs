use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct TransformWithModifier {
    pipeline: Pipeline
}

impl TransformWithModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return TransformWithModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Item for TransformWithModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(self.pipeline.process(ctx.clone()).await?))
    }
}
