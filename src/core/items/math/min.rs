use std::cmp::{max};
use async_trait::async_trait;
use teo_teon::value::Value;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct MinItem {
    argument: Value
}

impl MinItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for MinItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(max(v, argument.as_i32().unwrap()))),
            Value::I64(v) => ctx.with_value(Value::I64(max(v, argument.as_i64().unwrap()))),
            Value::F32(v) => ctx.with_value(Value::F32(if v >= argument.as_f32().unwrap() { v } else { argument.as_f32().unwrap() })),
            Value::F64(v) => ctx.with_value(Value::F64(if v >= argument.as_f64().unwrap() { v } else { argument.as_f64().unwrap() })),
            _ => Err(ctx.internal_server_error("min: value is not number"))?,
        })
    }
}
