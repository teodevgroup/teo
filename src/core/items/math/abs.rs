use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct AbsItem {}

impl AbsItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for AbsItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(v.abs())),
            Value::I64(v) => ctx.with_value(Value::I64(v.abs())),
            Value::F32(v) => ctx.with_value(Value::F32(v.abs())),
            Value::F64(v) => ctx.with_value(Value::F64(v.abs())),
            _ => Err(ctx.internal_server_error("abs: value is not a number"))?,
        })
    }
}
