use async_trait::async_trait;
use crate::core::env::position::Position;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct WhenManyResultsModifier {
    pipeline: Pipeline
}

impl WhenManyResultsModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenManyResultsModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenManyResultsModifier {

    fn name(&self) -> &'static str {
        "whenManyResults"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.object.env().position() {
            Some(Position::RootMany) | Some(Position::NestedMany) => self.pipeline.process(ctx.clone()).await,
            _ => ctx,
        }
    }
}
