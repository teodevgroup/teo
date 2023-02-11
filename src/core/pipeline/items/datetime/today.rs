use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct TodayModifier {}

impl TodayModifier {
    pub fn new() -> Self {
        return TodayModifier {};
    }
}

#[async_trait]
impl Item for TodayModifier {

    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        context.with_value(Value::Date(Utc::now().date_naive()))
    }
}
