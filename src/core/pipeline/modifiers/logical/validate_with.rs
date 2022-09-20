use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ValidateWithModifier {
    pipeline: Pipeline
}

impl ValidateWithModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return ValidateWithModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for ValidateWithModifier {

    fn name(&self) -> &'static str {
        "validateWith"
    }

    async fn call(&self, ctx: Context) -> Context {
        let new_ctx = self.pipeline.process(ctx.clone()).await;
        if new_ctx.is_valid() {
            ctx
        } else {
            ctx.invalid(new_ctx.invalid_reason().unwrap())
        }
    }
}
