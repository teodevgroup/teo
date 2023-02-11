use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct HasPrefixModifier {
    prefix: Value
}

impl HasPrefixModifier {
    pub fn new(prefix: impl Into<Value>) -> Self {
        Self { prefix: prefix.into() }
    }
}

#[async_trait]
impl Item for HasPrefixModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
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
