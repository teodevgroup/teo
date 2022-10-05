use async_trait::async_trait;
use crate::core::env::position::Position;
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
        match ctx.object.env().position() {
            Some(Position::NestedSingle) => self.pipeline.process(ctx.clone()).await,
            _ => ctx,
        }
    }
}
