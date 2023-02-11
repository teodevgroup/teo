use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct ExistsModifier {}

impl ExistsModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ExistsModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        if ctx.value.is_null() {
            ctx.invalid("Value does not exist.")
        } else {
            ctx
        }
    }
}
