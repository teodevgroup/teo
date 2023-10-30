use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct InvalidItem { }

impl InvalidItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for InvalidItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Err(ctx.with_invalid("value is invalid"))
    }
}
