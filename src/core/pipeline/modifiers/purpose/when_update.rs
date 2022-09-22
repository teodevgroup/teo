use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Purpose::Update;

#[derive(Debug, Clone)]
pub struct WhenUpdateModifier {
    pipeline: Pipeline
}

impl WhenUpdateModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenUpdateModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenUpdateModifier {

    fn name(&self) -> &'static str {
        "whenUpdate"
    }

    async fn call(&self, ctx: Context) -> Context {
        if ctx.purpose == Update {
            self.pipeline.process(ctx.clone()).await
        } else {
            ctx
        }
    }
}
