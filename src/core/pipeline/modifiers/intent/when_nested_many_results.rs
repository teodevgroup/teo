use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context, Intent};
use crate::core::pipeline::context::Intent::Create;

#[derive(Debug, Clone)]
pub struct WhenNestedManyResultsModifier {
    pipeline: Pipeline
}

impl WhenNestedManyResultsModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenNestedManyResultsModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenNestedManyResultsModifier {

    fn name(&self) -> &'static str {
        "whenNestedManyResults"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.intent {
            Intent::NestedManyResult(_) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}
