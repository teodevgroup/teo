use async_trait::async_trait;
use crate::core::action::r#type::ActionType;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Intent::{ManyResult, NestedManyResult, NestedSingleResult, SingleResult, Update};

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
        match ctx.intent {
            Update => self.pipeline.process(ctx.clone()).await,
            SingleResult(a) => if a == ActionType::Create {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            ManyResult(a) => if (a == ActionType::Update) || (a == ActionType::UpdateMany) {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            NestedSingleResult(a) => if a == ActionType::Update {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            NestedManyResult(a) => if (a == ActionType::Update) || (a == ActionType::UpdateMany) {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            _ => ctx
        }
    }
}
