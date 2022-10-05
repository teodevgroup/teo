use async_trait::async_trait;
use crate::core::action::r#type::ActionType;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct WhenUpdateModifier {
    pipeline: Pipeline
}

impl WhenUpdateModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenUpdateModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenUpdateModifier {

    fn name(&self) -> &'static str {
        "whenUpdate"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if let Some(intent) = ctx.object.env().intent() {
            if intent.is_update() {
                return self.pipeline.process(ctx.clone()).await;
            }
        }
        ctx
    }
}
