use async_trait::async_trait;
use crate::core::action::r#type::ActionType;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

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
        match ctx.object.env().action() {
            Some(ActionType::Create) | Some(ActionType::CreateMany) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}
