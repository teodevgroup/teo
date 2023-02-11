use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct IsEvenModifier {}

impl IsEvenModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsEvenModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::I32(v) => if v % 2 == 0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not even")) },
            Value::I64(v) => if v % 2 == 0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not even")) },
            Value::F32(v) => if v % 2.0 == 0.0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not even")) },
            Value::F64(v) => if v % 2.0 == 0.0 { Ok(ctx) } else { Err(ctx.with_invalid("value is not even")) },
            _ => Err(ctx.internal_server_error("isEven: value is not number"))
        }
    }
}
