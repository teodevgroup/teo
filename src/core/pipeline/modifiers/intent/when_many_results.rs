use async_trait::async_trait;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct WhenManyResultsModifier {
    pipeline: Pipeline
}

impl WhenManyResultsModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenManyResultsModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenManyResultsModifier {

    fn name(&self) -> &'static str {
        "whenManyResults"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if let Some(intent) = ctx.object.as_ref().unwrap().env().intent() {
            if intent.is_many() {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
        } else {
            ctx
        }
    }
}
