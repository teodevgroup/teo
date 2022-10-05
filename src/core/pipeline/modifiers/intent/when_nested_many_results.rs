use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context};

#[derive(Debug, Clone)]
pub struct WhenNestedManyResultsModifier {
    pipeline: Pipeline
}

impl WhenNestedManyResultsModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenNestedManyResultsModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenNestedManyResultsModifier {

    fn name(&self) -> &'static str {
        "whenNestedManyResults"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if let Some(intent) = ctx.object.env().intent() {
            if intent.is_nested_many() {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
        } else {
            ctx
        }
    }
}
