use async_trait::async_trait;
use crate::core::action::Action;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct RedirectItem {
    action: Action,
}

impl RedirectItem {
    pub(crate) fn new(action: Action) -> Self {
        RedirectItem {
            action
        }
    }
}

#[async_trait]
impl Item for RedirectItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Ok(ctx.redirect(self.action))
    }
}
