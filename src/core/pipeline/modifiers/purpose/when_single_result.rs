use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context, Purpose};
use crate::core::pipeline::context::Purpose::Create;

#[derive(Debug, Clone)]
pub struct WhenSingleResultModifier {
    pipeline: Pipeline
}

impl WhenSingleResultModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenSingleResultModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenSingleResultModifier {

    fn name(&self) -> &'static str {
        "whenSingleResult"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.purpose {
            Purpose::SingleResult(_) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}
