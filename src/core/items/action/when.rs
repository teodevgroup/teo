use async_trait::async_trait;
use crate::core::action::Action;
use crate::core::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
use crate::core::error::Error;

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
        let action = if ctx.action.is_empty() {
            match ctx.object.as_ref() {
                Some(object) => object.action(),
                None => Err(Error::internal_server_error("when: action not found"))?
            }
        } else {
            ctx.action
        };
        if action.passes(&self.actions) {
            let result = self.pipeline.process_with_ctx_result(ctx.clone()).await?;
            Ok(ctx.with_value(result.value.clone()).with_action(result.action))
        } else {
            Ok(ctx)
        }
    }
}
