use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::value::Value;

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
impl Modifier for PassedModifier {

    fn name(&self) -> &'static str {
        "passed"
    }

    async fn call(&self, context: Context) -> Context {
        if self.pipeline.process(context.clone()).await.is_valid() {
            context.alter_value(Value::Bool(true))
        } else {
            context.alter_value(Value::Bool(false))
        }
    }
}
