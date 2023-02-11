use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct NowModifier {}

impl NowModifier {
    pub fn new() -> Self {
        NowModifier {}
    }
}

#[async_trait]
impl Item for NowModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        context.with_value(Value::DateTime(Utc::now()))
    }
}
