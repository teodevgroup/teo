use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct IsNumericItem {}

impl IsNumericItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsNumericItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                for c in s.chars() {
                    if !c.is_numeric() {
                        return Err(ctx.with_invalid("value is not numeric"));
                    }
                }
                Ok(ctx)
            }
            None => {
                Err(ctx.internal_server_error("isNumeric: value is not string"))
            }
        }
    }
}
