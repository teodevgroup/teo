use async_trait::async_trait;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

#[derive(Debug, Clone)]
pub struct PassedModifier {
    pipeline: Pipeline
}

impl PassedModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return PassedModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Item for PassedModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        if self.pipeline.process(context.clone()).await.is_valid() {
            context.with_value(Value::Bool(true))
        } else {
            context.with_value(Value::Bool(false))
        }
    }
}
