use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use teo_teon::value::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct IsSuffixOfItem {
    full: Value
}

impl IsSuffixOfItem {
    pub fn new(full: impl Into<Value>) -> Self {
        Self { full: full.into() }
    }
}

#[async_trait]
impl Item for IsSuffixOfItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.with_invalid("isSuffixOf: value is not string")),
            Some(s) => {
                let arg = self.full.resolve(ctx.clone()).await?;
                let full = arg.as_str().unwrap();
                if full.ends_with(s) {
                    Ok(ctx)
                } else {
                    Err(ctx.internal_server_error("value is not suffix"))
                }
            }
        }
    }
}
