use async_trait::async_trait;
use crate::core::env::position::Position;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context};

#[derive(Debug, Clone)]
pub struct WhenSingleResultModifier {
    pipeline: Pipeline
}

impl WhenSingleResultModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenSingleResultModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenSingleResultModifier {

    fn name(&self) -> &'static str {
        "whenSingleResult"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.object.env().position() {
            Some(Position::RootSingle) | Some(Position::NestedSingle) => self.pipeline.process(ctx.clone()).await,
            _ => ctx,
        }
    }
}
