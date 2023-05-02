use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct AtItem {
    index: Value
}

impl AtItem {
    pub fn new(index: impl Into<Value>) -> Self {
        Self { index: index.into() }
    }
}

#[async_trait]
impl Item for AtItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_vec() {
            None => Err(ctx.internal_server_error("at: value is not vector")),
            Some(v) => {
                let arg = self.index.resolve(ctx.clone()).await?;
                let index = arg.as_i32().unwrap() as usize;
                let new_path = ctx.path.as_ref() + index as usize;
                Ok(ctx.with_value(v.get(index).unwrap().clone())
                   .with_path(new_path))
            }
        }
    }
}
