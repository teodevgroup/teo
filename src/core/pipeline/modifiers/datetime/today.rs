use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
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

    async fn call(&self, context: Context) -> Context {
        context.alter_value(Value::Date(Utc::today()))
    }
}
