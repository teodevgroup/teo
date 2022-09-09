use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;


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

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        if let Some(_) = &stage.value() {
            return self.pipeline.process(stage.clone(), object).await;
        } else {
            return stage;
        }
    }
}
