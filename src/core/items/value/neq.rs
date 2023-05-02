use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct NeqItem {
    argument: Value
}

impl NeqItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for NeqItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let rhs = self.argument.resolve(ctx.clone()).await?;
        if rhs != ctx.value {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("neq: value is equal to rhs"))
        }
    }
}
