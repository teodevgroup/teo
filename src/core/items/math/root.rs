use async_trait::async_trait;
use num_integer::Roots;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct RootItem {
    argument: Value
}

impl RootItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for RootItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        let exp = argument.as_i32().unwrap() as u32;
        Ok(match ctx.get_value() {
            Value::I32(v) => ctx.with_value(Value::I32(v.nth_root(exp))),
            Value::I64(v) => ctx.with_value(Value::I64(v.nth_root(exp))),
            _ => Err(ctx.internal_server_error("Value is not integer."))?,
        })
    }
}
