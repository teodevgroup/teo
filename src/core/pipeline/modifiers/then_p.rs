use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::stage::Stage::{Value as StageValue, ConditionTrue};


#[derive(Debug, Clone)]
pub struct ThenPModifier {
    pipeline: Pipeline
}

impl ThenPModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ThenPModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for ThenPModifier {

    fn name(&self) -> &'static str {
        "then_p"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        return match stage {
            ConditionTrue(value) => {
                self.pipeline.process(StageValue(value), object).await
            }
            _ => {
                stage
            }
        }
    }
}
