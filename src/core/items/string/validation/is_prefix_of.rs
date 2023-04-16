use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct IsPrefixOfItem {
    full: Value
}

impl IsPrefixOfItem {
    pub fn new(full: impl Into<Value>) -> Self {
        Self { full: full.into() }
    }
}

#[async_trait]
impl Item for IsPrefixOfItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.with_invalid("isPrefixOf: value is not string")),
            Some(s) => {
                let arg = self.full.resolve(ctx.clone()).await?;
                let full = arg.as_str().unwrap();
                if full.starts_with(s) {
                    Ok(ctx)
                } else {
                    Err(ctx.internal_server_error("value is not prefix"))
                }
            }
        }
    }
}
