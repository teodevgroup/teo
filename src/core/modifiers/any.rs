use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Clone)]
pub struct AnyModifier {
    pipelines: Vec<Pipeline>
}

impl AnyModifier {
    pub fn new<F: Fn(&mut Pipeline)>(build: F) -> Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        let pipelines: Vec<Pipeline> = pipeline.modifiers.iter().map(|modifier| {
            let mut p = Pipeline::new();
            p.modifiers.push(modifier.clone());
            p
        }).collect();
        return AnyModifier { pipelines };
    }
}


#[async_trait]
impl Modifier for AnyModifier {

    fn name(&self) -> &'static str {
        "any"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        for pipeline in &self.pipelines {
            let result = pipeline._process(stage.clone(), object).await;
            if let Some(_) = result.value() {
                return stage.clone()
            }
        }
        Stage::Invalid("Invalid in any call.".to_string())
    }
}
