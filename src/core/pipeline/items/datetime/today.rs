use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct TodayItem {}

impl TodayItem {
    pub fn new() -> Self {
        return TodayItem {};
    }
}

#[async_trait]
impl Item for TodayItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::Date(Utc::now().date_naive())))
    }
}
