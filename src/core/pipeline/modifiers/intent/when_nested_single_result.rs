use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context};

#[derive(Debug, Clone)]
pub struct WhenNestedSingleResultModifier {
    pipeline: Pipeline
}

impl WhenNestedSingleResultModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenNestedSingleResultModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenNestedSingleResultModifier {

    fn name(&self) -> &'static str {
        "whenNestedSingleResult"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if let Some(intent) = ctx.object.as_ref().unwrap().env().intent() {
            if intent.is_nested_single() {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
        } else {
            ctx
        }
    }
}
