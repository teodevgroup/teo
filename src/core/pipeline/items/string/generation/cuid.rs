use async_trait::async_trait;
use cuid::cuid;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct CUIDItem {}

impl CUIDItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for CUIDItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(ctx.with_value(Value::String(cuid().unwrap())))
    }
}
