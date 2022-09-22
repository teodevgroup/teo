use async_trait::async_trait;
use crate::core::action::r#type::ActionType;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Intent::{ManyResult, SingleResult, Update};

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

    async fn call(&self, ctx: Context) -> Context {
        match ctx.intent {
            Update => self.pipeline.process(ctx.clone()).await,
            SingleResult(a) => if a == ActionType::Create {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            ManyResult(a) => if a == ActionType::UpdateMany {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            _ => ctx
        }
    }
}
