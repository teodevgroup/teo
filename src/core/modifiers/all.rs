use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Clone)]
pub struct AllModifier {
    pipelines: Vec<Pipeline>
}

impl AllModifier {
    pub fn new<F: Fn(&mut Pipeline)>(build: F) -> Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        let pipelines: Vec<Pipeline> = pipeline.modifiers.iter().map(|modifier| {
            let mut p = Pipeline::new();
            p.modifiers.push(modifier.clone());
            p
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
            let result = pipeline._process(stage.clone(), object).await;
            if let Some(_) = result.invalid() {
                return Stage::Invalid("Invalid in all call.".to_string());
            }
        }
        stage.clone()
    }
}
