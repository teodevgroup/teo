use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct TransformWithItem {
    pipeline: Pipeline
}

impl TransformWithItem {
    pub fn new(pipeline: Pipeline) -> Self {
        return TransformWithItem {
            pipeline
        };
    }
}

#[async_trait]
impl Item for TransformWithItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Ok(ctx.with_value(self.pipeline.process(ctx.clone()).await?))
    }
}
