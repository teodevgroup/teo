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
    pub fn new<F: Fn(&mut Pipeline)>(builders: Vec<F>) -> Self {
        let pipelines = builders.iter().map(|b| {
            let mut pipeline = Pipeline::new();
            b(&mut pipeline);
            pipeline
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
