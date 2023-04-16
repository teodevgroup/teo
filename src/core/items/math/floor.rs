use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct FloorItem {}

impl FloorItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for FloorItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::F32(v) => Ok(ctx.with_value(Value::F32(v.floor()))),
            Value::F64(v) => Ok(ctx.with_value(Value::F64(v.floor()))),
            Value::I32(_) | Value::I64(_) => Ok(ctx.clone()),
            _ => Err(ctx.internal_server_error("floor: value is not number"))?
        }
    }
}
