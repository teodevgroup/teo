use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct ReverseItem {}

impl ReverseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ReverseItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match &ctx.value {
            Value::String(s) => Ok(ctx.with_value(Value::String(s.chars().rev().collect::<String>()))),
            Value::Vec(v) => Ok(ctx.with_value(Value::Vec(v.into_iter().rev().map(|v| v.clone()).collect()))),
            _ => Err(ctx.internal_server_error("reverse: value is not vector"))
        }
    }
}
