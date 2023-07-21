use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct OneOfItem {
    argument: Value
}

impl OneOfItem {
    pub fn new(argument: Value) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for OneOfItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let arg = self.argument.resolve(ctx.clone()).await?;
        let list = arg.as_vec().unwrap();
        if list.iter().find(|item| {
            **item == ctx.value
        }).is_some() {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("oneOf: value is not one of valid ones"))
        }
    }
}
