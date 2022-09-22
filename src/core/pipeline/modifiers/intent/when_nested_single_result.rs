use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context, Intent};
use crate::core::pipeline::context::Intent::Create;

#[derive(Debug, Clone)]
pub struct WhenNestedSingleResultModifier {
    pipeline: Pipeline
}

impl WhenNestedSingleResultModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenNestedSingleResultModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenNestedSingleResultModifier {

    fn name(&self) -> &'static str {
        "whenNestedSingleResult"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.intent {
            Intent::NestedSingleResult(_) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}
