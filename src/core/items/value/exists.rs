use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct ExistsItem {}

impl ExistsItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ExistsItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if ctx.value.is_null() {
            Err(ctx.with_invalid("exists: value does not exist"))
        } else {
            Ok(ctx)
        }
    }
}
