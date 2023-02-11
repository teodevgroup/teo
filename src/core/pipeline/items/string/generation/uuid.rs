use async_trait::async_trait;
use uuid::Uuid;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct UUIDModifier {}

impl UUIDModifier {
    pub fn new() -> Self {
        return UUIDModifier {};
    }
}

#[async_trait]
impl Item for UUIDModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        context.with_value(Value::String(Uuid::new_v4().to_string()))
    }
}
