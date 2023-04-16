use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct IsNullItem { }

impl IsNullItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for IsNullItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if ctx.value.is_null() {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("isNull: value is not null"))
        }
    }
}
