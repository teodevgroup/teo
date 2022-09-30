use async_trait::async_trait;
use crate::core::action::r#type::ActionType;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Intent::{Create, SingleResult, ManyResult, NestedSingleResult, NestedManyResult};

#[derive(Debug, Clone)]
pub struct WhenCreateModifier {
    pipeline: Pipeline
}

impl WhenCreateModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenCreateModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenCreateModifier {

    fn name(&self) -> &'static str {
        "whenCreate"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.intent {
            Create => self.pipeline.process(ctx.clone()).await,
            SingleResult(a) => if a == ActionType::Create {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            ManyResult(a) => if (a == ActionType::Create) || (a == ActionType::CreateMany) {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            NestedSingleResult(a) => if a == ActionType::Create {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            NestedManyResult(a) => if (a == ActionType::Create) || (a == ActionType::CreateMany) {
                self.pipeline.process(ctx.clone()).await
            } else {
                ctx
            }
            _ => ctx
        }
    }
}
