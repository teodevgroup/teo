use num_integer::Roots;

use async_trait::async_trait;
use crate::core::result::Result;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct SqrtItem { }

impl SqrtItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for SqrtItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(v.sqrt())),
            Value::I64(v) => ctx.with_value(Value::I64(v.sqrt())),
            Value::F32(v) => ctx.with_value(Value::F32(v.sqrt())),
            Value::F64(v) => ctx.with_value(Value::F64(v.sqrt())),
            _ => Err(ctx.internal_server_error("sqrt: value is not number"))?,
        })
    }
}
