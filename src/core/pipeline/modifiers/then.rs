use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::stage::Stage::{Value as StageValue, ConditionTrue};

#[derive(Debug, Clone)]
pub struct ThenModifier {
    pipeline: Pipeline
}

impl ThenModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ThenModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for ThenModifier {

    fn name(&self) -> &'static str {
        "then"
    }

    async fn call(&self, context: Context) -> Context {
        if context.is_condition_true() {
            self.pipeline.process(context)
        } else {
            context
        }
    }
}
