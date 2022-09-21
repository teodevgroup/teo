use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Stage::{ConditionFalse, ConditionTrue};

#[derive(Debug, Clone)]
pub struct NotModifier {
    pipeline: Pipeline
}

impl NotModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return NotModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for NotModifier {

    fn name(&self) -> &'static str {
        "not"
    }

    async fn call(&self, ctx: Context) -> Context {
        if self.pipeline.process(ctx.clone()).await.is_valid() {
            ctx.invalid("Condition is valid.")
        } else {
            ctx
        }
    }
}
