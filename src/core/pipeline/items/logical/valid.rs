
use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct ValidModifier { }

impl ValidModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for ValidModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        ctx
    }
}
