use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::stage::Stage::{Value as StageValue, ConditionTrue, ConditionFalse, Invalid};


#[derive(Debug, Clone)]
pub struct IfPModifier {
    pipeline: Pipeline
}

impl IfPModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return IfPModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for IfPModifier {

    fn name(&self) -> &'static str {
        "if_p"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        let result = self.pipeline.process(stage.clone(), object).await;
        match result {
            StageValue(value) => {
                return ConditionTrue(value);
            }
            Invalid(_) => {
                return ConditionFalse(stage.clone().value().unwrap());
            }
            _ => {
                panic!("Wrong value returned by pipeline.")
            }
        }
    }
}
