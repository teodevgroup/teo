use async_trait::async_trait;
use chrono::Utc;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct NowItem {}

impl NowItem {
    pub fn new() -> Self {
        NowItem { }
    }
}

#[async_trait]
impl Item for NowItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::DateTime(Utc::now())))
    }
}
