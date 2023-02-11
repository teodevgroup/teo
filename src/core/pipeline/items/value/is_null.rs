use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct IsNullModifier { }

impl IsNullModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for IsNullModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if ctx.value.is_null() {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("isNull: value is not null"))
        }
    }
}
