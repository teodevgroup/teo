
use async_trait::async_trait;

use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct ValidItem { }

impl ValidItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for ValidItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx)
    }
}
