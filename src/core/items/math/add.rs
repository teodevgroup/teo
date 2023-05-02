use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::prelude::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct AddItem {
    argument: Value
}

impl AddItem {
    pub fn new(argument: Value) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for AddItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(ctx.with_value_result(ctx.get_value() + argument)?)
    }
}
