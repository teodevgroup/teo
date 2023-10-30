use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use teo_teon::value::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct HasPrefixItem {
    prefix: Value
}

impl HasPrefixItem {
    pub fn new(prefix: impl Into<Value>) -> Self {
        Self { prefix: prefix.into() }
    }
}

#[async_trait]
impl Item for HasPrefixItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.internal_server_error("hasPrefix: value is not string")),
            Some(s) => {
                let arg = self.prefix.resolve(ctx.clone()).await?;
                let prefix = arg.as_str().unwrap();
                if s.starts_with(prefix) {
                    Ok(ctx)
                } else {
                    Err(ctx.with_invalid("value is not correctly prefixed"))
                }
            }
        }
    }
}
