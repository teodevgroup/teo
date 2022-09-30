use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

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

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        if context.is_condition_true() {
            self.pipeline.process(context).await
        } else {
            context
        }
    }
}
