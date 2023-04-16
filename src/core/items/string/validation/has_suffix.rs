use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct HasSuffixItem {
    suffix: Value
}

impl HasSuffixItem {
    pub fn new(suffix: impl Into<Value>) -> Self {
        Self { suffix: suffix.into() }
    }
}

#[async_trait]
impl Item for HasSuffixItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.with_invalid("hasSuffix: value is not string")),
            Some(s) => {
                let arg = self.suffix.resolve(ctx.clone()).await?;
                let suffix = arg.as_str().unwrap();
                if s.ends_with(suffix) {
                    Ok(ctx)
                } else {
                    Err(ctx.internal_server_error("value is not correctly suffixed"))
                }
            }
        }
    }
}
