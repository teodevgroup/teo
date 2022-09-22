use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context, Purpose};
use crate::core::pipeline::context::Purpose::Create;

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

    async fn call(&self, ctx: Context) -> Context {
        match ctx.purpose {
            Purpose::ManyResult(_) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}
