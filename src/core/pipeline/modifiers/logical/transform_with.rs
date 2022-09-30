use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

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
impl Modifier for TransformWithModifier {

    fn name(&self) -> &'static str {
        "transformWith"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let new_ctx = self.pipeline.process(ctx.clone()).await;
        ctx.alter_value(new_ctx.value)
    }
}
