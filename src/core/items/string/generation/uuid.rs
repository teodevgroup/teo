use async_trait::async_trait;
use uuid::Uuid;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct UUIDItem {}

impl UUIDItem {
    pub fn new() -> Self {
        return UUIDItem {};
    }
}

#[async_trait]
impl Item for UUIDItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::String(Uuid::new_v4().to_string())))
    }
}
