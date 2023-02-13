use async_trait::async_trait;
use crate::core::action::Action;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct WhenItem {
    actions: Vec<Action>,
    pipeline: Pipeline
}

impl WhenItem {
    pub(crate) fn new(actions: Vec<Action>, pipeline: Pipeline) -> Self {
        return WhenItem {
            actions,
            pipeline
        };
    }
}

#[async_trait]
impl Item for WhenItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let object_action = ctx.object.as_ref().unwrap().action();
        if object_action.passes(&self.actions) {
            Ok(ctx.with_value(self.pipeline.process(ctx.clone()).await?))
        } else {
            Ok(ctx)
        }
    }
}
