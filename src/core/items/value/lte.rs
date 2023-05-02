use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct LteItem {
    argument: Value
}

impl LteItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for LteItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let rhs = self.argument.resolve(ctx.clone()).await?;
        if ctx.value <= rhs {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("lte: value is not less than or equal to rhs"))
        }
    }
}
