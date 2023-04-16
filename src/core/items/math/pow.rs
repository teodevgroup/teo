use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct PowItem {
    argument: Value
}

impl PowItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for PowItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        let exp = argument.as_i32().unwrap() as u32;
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(v.pow(exp))),
            Value::I64(v) => ctx.with_value(Value::I64(v.pow(exp))),
            Value::F32(v) => ctx.with_value(Value::F32(v.powf(argument.as_f32().unwrap()))),
            Value::F64(v) => ctx.with_value(Value::F64(v.powf(argument.as_f64().unwrap()))),
            _ => Err(ctx.internal_server_error("pow: value is not number"))?,
        })
    }
}
