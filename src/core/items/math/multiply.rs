use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct MultiplyItem {
    argument: Value
}

impl MultiplyItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for MultiplyItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(ctx.with_value_result(ctx.get_value() * argument)?)
    }
}
