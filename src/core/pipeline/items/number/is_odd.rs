use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct IsOddItem {}

impl IsOddItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsOddItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::I32(v) => if v % 2 == 1 { Ok(ctx) } else { Err(ctx.with_invalid("value is not odd")) },
            Value::I64(v) => if v % 2 == 1 { Ok(ctx) } else { Err(ctx.with_invalid("value is not odd")) },
            Value::F32(v) => if v % 2.0 == 1.0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not odd")) },
            Value::F64(v) => if v % 2.0 == 1.0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not odd")) },
            _ => Err(ctx.internal_server_error("isOdd: value is not number"))
        }
    }
}
