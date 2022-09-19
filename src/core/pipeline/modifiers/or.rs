use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;


#[derive(Debug, Clone)]
pub struct OrModifier {
    pipeline: Pipeline
}

impl OrModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        return OrModifier { pipeline: pipeline.build() };
    }
}

#[async_trait]
impl Modifier for OrModifier {

    fn name(&self) -> &'static str {
        "or"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        if let Some(_) = &stage.value() {
            return stage
        }
        return self.pipeline.process(stage.clone(), object).await
    }
}
