use std::cmp::{min};
use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct MaxItem {
    argument: Value
}

impl MaxItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for MaxItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(min(v, argument.as_i32().unwrap()))),
            Value::I64(v) => ctx.with_value(Value::I64(min(v, argument.as_i64().unwrap()))),
            Value::F32(v) => ctx.with_value(Value::F32(if v <= argument.as_f32().unwrap() { v } else { argument.as_f32().unwrap() })),
            Value::F64(v) => ctx.with_value(Value::F64(if v <= argument.as_f64().unwrap() { v } else { argument.as_f64().unwrap() })),
            _ => Err(ctx.internal_server_error("max: value is not number"))?,
        })
    }
}
