use std::sync::{Arc};
use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::stage::Stage::{Value as StageValue, ConditionTrue};


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

    async fn call(&self, stage: Stage, _object: Arc<Object>) -> Stage {
        return match stage {
            ConditionTrue(value) => {
                self.pipeline._process(StageValue(value), _object).await
            }
            _ => {
                stage
            }
        }
    }
}
