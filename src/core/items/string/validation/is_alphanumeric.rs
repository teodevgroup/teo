use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct IsAlphanumericItem {}

impl IsAlphanumericItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsAlphanumericItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                for c in s.chars() {
                    if !c.is_alphanumeric() {
                        return Err(ctx.with_invalid("value is not alphanumeric"));
                    }
                }
                Ok(ctx)
            }
            None => {
                Err(ctx.internal_server_error("isAlphanumeric: value is not string"))
            }
        }
    }
}
