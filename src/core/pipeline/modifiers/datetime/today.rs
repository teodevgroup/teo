use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct TodayModifier {}

impl TodayModifier {
    pub fn new() -> Self {
        return TodayModifier {};
    }
}

#[async_trait]
impl Modifier for TodayModifier {

    fn name(&self) -> &'static str {
        "today"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        context.alter_value(Value::Date(Utc::today()))
    }
}
