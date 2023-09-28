use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use teo_teon::value::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct GtItem {
    argument: Value
}

impl GtItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for GtItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let rhs = self.argument.resolve(ctx.clone()).await?;
        if ctx.value > rhs {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("gt: value is not greater than rhs"))
        }
    }
}
