use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct IsNumericModifier {}

impl IsNumericModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsNumericModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                for c in s.chars() {
                    if !c.is_numeric() {
                        return Err(ctx.invalid("value is not numeric"));
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
