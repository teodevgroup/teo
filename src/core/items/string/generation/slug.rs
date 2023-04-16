use async_trait::async_trait;
use cuid::slug;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct SlugItem {}

impl SlugItem {
    pub fn new() -> Self {
        SlugItem {}
    }
}

#[async_trait]
impl Item for SlugItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::String(slug().unwrap())))
    }
}
