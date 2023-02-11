use async_trait::async_trait;
use num_integer::Roots;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct RootModifier {
    argument: Value
}

impl RootModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for RootModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        let exp = argument.as_i32().unwrap() as u32;
        match ctx.value {
            Value::I32(v) => ctx.with_value(Value::I32(v.nth_root(exp))),
            Value::I64(v) => ctx.with_value(Value::I64(v.nth_root(exp))),
            Value::F32(_v) => ctx.invalid("F32 value doesn't support nth root yet."),
            Value::F64(_v) => ctx.invalid("F64 value doesn't support nth root yet."),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
