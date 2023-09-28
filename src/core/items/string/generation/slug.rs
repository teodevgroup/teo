use async_trait::async_trait;
use cuid::slug;
use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct SlugItem {}

impl SlugItem {
    pub fn new() -> Self {
        SlugItem {}
    }
}

#[async_trait]
impl Item for SlugItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Ok(ctx.with_value(Value::String(slug().unwrap())))
    }
}
