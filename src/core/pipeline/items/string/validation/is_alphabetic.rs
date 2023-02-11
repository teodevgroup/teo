use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct IsAlphabeticModifier {}

impl IsAlphabeticModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsAlphabeticModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.internal_server_error("isAlphabetic: value is not string")),
            Some(s) => {
                for c in s.chars() {
                    if !c.is_alphabetic() {
                        return Err(ctx.invalid("value is not alphabetic"));
                    }
                }
                Ok(ctx)
            }
        }
    }
}
