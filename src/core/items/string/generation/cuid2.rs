use async_trait::async_trait;
use cuid2::create_id;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct CUID2Item {}

impl CUID2Item {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for CUID2Item {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::String(create_id())))
    }
}
