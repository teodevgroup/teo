use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct IsFalseItem { }

impl IsFalseItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for IsFalseItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let valid = match ctx.value.as_bool() {
            Some(b) => !b,
            None => false
        };
        if valid {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("isFalse: value is not false"))
        }
    }
}
