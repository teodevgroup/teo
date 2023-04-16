use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct TrimItem {}

impl TrimItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for TrimItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::String(ref s) => Ok(ctx.with_value(Value::String(s.trim().to_owned()))),
            _ => Err(ctx.internal_server_error("trim: value is not string"))
        }
    }
}
