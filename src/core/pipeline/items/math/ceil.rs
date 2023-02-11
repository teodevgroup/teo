use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct CeilItem {}

impl CeilItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for CeilItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(match ctx.get_value() {
            Value::F32(v) => ctx.with_value(Value::F32(v.ceil())),
            Value::F64(v) => ctx.with_value(Value::F64(v.ceil())),
            Value::I32(_) | Value::I64(_) => ctx.clone(),
            _ => Err(ctx.internal_server_error("ceil: value is not number"))?
        })
    }
}
