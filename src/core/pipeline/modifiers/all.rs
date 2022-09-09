use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::stage::Stage;

#[derive(Debug, Clone)]
pub struct AllModifier {
    pipelines: Vec<Pipeline>
}

impl AllModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        let pipelines: Vec<Pipeline> = pipeline.modifiers.iter().map(|modifier| {
            let mut p = PipelineBuilder::new();
            p.modifiers.push(modifier.clone());
            p.build()
        }).collect();
        return AllModifier { pipelines };
    }
}


#[async_trait]
impl Modifier for AllModifier {

    fn name(&self) -> &'static str {
        "all"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        for pipeline in &self.pipelines {
            let result = pipeline.process(stage.clone(), object).await;
            if let Some(_) = result.invalid() {
                return Stage::Invalid("Invalid in all call.".to_string());
            }
        }
        stage.clone()
    }
}
