use async_trait::async_trait;
use cuid::cuid;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct CUIDItem {}

impl CUIDItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for CUIDItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Ok(ctx.with_value(Value::String(cuid().unwrap())))
    }
}
