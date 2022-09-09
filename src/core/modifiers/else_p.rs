use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::stage::Stage;
use crate::core::pipeline::stage::Stage::{Value as StageValue, ConditionFalse};


#[derive(Debug, Clone)]
pub struct ElsePModifier {
    pipeline: Pipeline
}

impl ElsePModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ElsePModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for ElsePModifier {

    fn name(&self) -> &'static str {
        "else_p"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        return match stage {
            ConditionFalse(value) => {
                self.pipeline.process(StageValue(value), object).await
            }
            _ => {
                stage
            }
        }
    }
}
