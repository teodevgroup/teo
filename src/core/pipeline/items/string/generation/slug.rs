use async_trait::async_trait;
use cuid::slug;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct SlugModifier {}

impl SlugModifier {
    pub fn new() -> Self {
        SlugModifier {}
    }
}

#[async_trait]
impl Item for SlugModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        ctx.with_value(Value::String(slug().unwrap()))
    }
}
