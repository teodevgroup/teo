use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ElseModifier {
    pipeline: Pipeline
}

impl ElseModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ElseModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for ElseModifier {

    fn name(&self) -> &'static str {
        "else"
    }

    async fn call(&self, context: Context) -> Context {
        if context.is_condition_false() {
            self.pipeline.process(context).await
        } else {
            context
        }
    }
}
