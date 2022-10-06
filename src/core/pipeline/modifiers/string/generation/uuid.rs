use async_trait::async_trait;
use uuid::Uuid;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct UUIDModifier {}

impl UUIDModifier {
    pub fn new() -> Self {
        return UUIDModifier {};
    }
}

#[async_trait]
impl Modifier for UUIDModifier {

    fn name(&self) -> &'static str {
        "uuid"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        context.alter_value(Value::String(Uuid::new_v4().to_string()))
    }
}
