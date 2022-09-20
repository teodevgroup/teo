use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct DoModifier {
    pipeline: Pipeline
}

impl DoModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        return DoModifier { pipeline: pipeline.build() };
    }
}

#[async_trait]
impl Modifier for DoModifier {

    fn name(&self) -> &'static str {
        "do"
    }

    async fn call(&self, context: Context) -> Context {
        self.pipeline.process(context).await
    }
}
