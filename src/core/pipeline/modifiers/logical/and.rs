use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;


#[derive(Debug, Clone)]
pub struct AndModifier {
    pipeline: Pipeline
}

impl AndModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        return AndModifier { pipeline: pipeline.build() };
    }
}

#[async_trait]
impl Modifier for AndModifier {

    fn name(&self) -> &'static str {
        "and"
    }

    async fn call(&self, ctx: Context) -> Context {
        if !ctx.value.is_null() {
            ctx
        } else {
            self.pipeline.process(ctx).await
        }
    }
}
