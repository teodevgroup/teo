use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct TrimModifier {}

impl TrimModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for TrimModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::String(ref s) => ctx.with_value(Value::String(s.trim().to_owned())),
            _ => ctx.internal_server_error("Value is not string.")
        }
    }
}
